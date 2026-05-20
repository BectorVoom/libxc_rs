//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 758/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk758<F: Float>(t33: F, t265: F, t502: F, t2155: F, t3801: F, t1298: F, t1300: F, t198: F, t336: F, t5023: F, t7193: F, t7669: F, t2159: F, t57: F, t606: F, t7214: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t7673 = t2155 * t3801;
    let t7677 = piecewise3::<F>(t503, t1300 * t198 * t336 * t7669 - t1298 * t5023 * t7673, t7193);
    let t7682 = piecewise3::<F>(t400, t7214, -t2159 * t606 / F::new(2.0) + t7677 * t57 / F::new(2.0));
    (t7673, t7677, t7682)
}
