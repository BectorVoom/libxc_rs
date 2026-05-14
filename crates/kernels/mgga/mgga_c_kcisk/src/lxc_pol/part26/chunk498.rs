//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 498/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk498<F: Float>(t3598: F, t420: F, t1173: F, t1361: F, t3571: F, t1167: F, t1171: F, t1170: F, t317: F, t305: F, t303: F, t1379: F, t311: F, t313: F, t1187: F, t827: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3599 = t3598 * t420;
    let t3602 = t1173 * t1361;
    let t3611 = 0.55033333333333333333e-2 * t3571;
    let t3626 = 0.23744444444444444444e-1 * t3571;
    let t3634 = t1167 * t1171;
    let t3637 = t1170 * t317;
    let t3638 = 1.0 / t3637;
    let t3639 = t305 * t3638;
    let t3646 = 0.39862222222222222223e0 * t3571;
    let t3651 = 1.0/f64::sqrt(t303);
    let t3657 = t311 * t1379 * t313;
    let t3658 = 0.13692777777777777778e0 * t3657;
    let t3659 = t827 * t1187;
    (t3599, t3602, t3611, t3626, t3634, t3638, t3639, t3646, t3651, t3657, t3658, t3659)
}
