//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 717/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk717<F: Float>(t33: F, t265: F, t502: F, t1300: F, t1832: F, t198: F, t336: F, t5023: F, t7673: F, t7855: F, t8220: F, t1469: F, t2159: F, t57: F, t7876: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t8227 = piecewise3::<F>(t503, t1300 * t198 * t336 * t8220 - t1832 * t5023 * t7673, t7855);
    let t8232 = piecewise3::<F>(t400, t7876, -t2159 * t1469 / F::new(2.0) + t8227 * t57 / F::new(2.0));
    (t8227, t8232)
}
