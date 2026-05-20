//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 747/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk747<F: Float>(t33: F, t265: F, t502: F, t1940: F, t8490: F, t8494: F, t8542: F, t57: F, t8546: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t8552 = t1940 * t8490 * t33 / F::new(2.0) - t1940 * t8494 * t33 / F::new(2.0);
    let t8553 = piecewise3::<F>(t503, F::new(0.0), t8542);
    let t8556 = piecewise3::<F>(t400, t8552, t8553 * t57 / F::new(2.0));
    let t8557 = t8546 + t8556;
    (t8553, t8557)
}
