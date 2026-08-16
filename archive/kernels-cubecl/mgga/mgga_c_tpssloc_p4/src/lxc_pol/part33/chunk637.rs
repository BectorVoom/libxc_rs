//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 637/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk637<F: Float>(t1088: F, t5979: F, t123: F, t3237: F, t4721: F, t5973: F, t5977: F, t423: F, t1671: F, t4740: F, t1670: F, t1118: F) -> (F, F, F, F, F, F, F) {
    let t5980 = t1088 * t5979;
    let t5981 = t123 * t5980;
    let t5983 = t3237 - F::cast_from(0.11872222222222222222e-1_f64) * t4721 - F::cast_from(0.11872222222222222222e-1_f64) * t5973 + F::cast_from(0.35616666666666666666e-1_f64) * t5977 + F::cast_from(0.17808333333333333333e-1_f64) * t5981;
    let t5985 = F::cast_from(0.621814e-1_f64) * t5983 * t423;
    let t5987 = F::cast_from(2.0_f64) * t4740 * t1671;
    let t5988 = t1670 * t1670;
    let t5989 = t5988 * t1118;
    (t5980, t5981, t5983, t5985, t5987, t5988, t5989)
}
