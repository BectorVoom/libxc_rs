//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 439/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk439<F: Float>(t3598: F, t420: F, t3571: F, t1170: F, t317: F, t305: F, t303: F, t1379: F, t311: F, t313: F, t1311: F, t79: F, t320: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3599 = t3598 * t420;
    let t3611 = 0.55033333333333333333e-2 * t3571;
    let t3626 = 0.23744444444444444444e-1 * t3571;
    let t3637 = t1170 * t317;
    let t3638 = 1.0 / t3637;
    let t3639 = t305 * t3638;
    let t3646 = 0.39862222222222222223e0 * t3571;
    let t3651 = 1.0/f64::sqrt(t303);
    let t3657 = t311 * t1379 * t313;
    let t3658 = 0.13692777777777777778e0 * t3657;
    let t3661 = t79 * t1311;
    let t3675 = t1170 * t1170;
    let t3676 = 1.0 / t3675;
    let t3677 = t305 * t3676;
    let t3678 = t320 * t320;
    let t3679 = 1.0 / t3678;
    let t3683 = 0.12361111111111111111e-1 * t3571;
    (t3599, t3611, t3626, t3638, t3639, t3646, t3651, t3657, t3658, t3661, t3675, t3676, t3677, t3678, t3679, t3683)
}
