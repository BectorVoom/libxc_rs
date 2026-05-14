//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 451/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk451<F: Float>(t1190: F, t1191: F, t3639: F, t3571: F, t303: F, t3559: F, t1180: F, t3587: F, t1379: F, t311: F, t313: F, t1187: F, t827: F, t1311: F, t79: F, t3575: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3640 = t1190 * t1190;
    let t3641 = t3640 * t1191;
    let t3643 = 2.0 * t3639 * t3641;
    let t3646 = 0.39862222222222222223e0 * t3571;
    let t3651 = 1.0/f64::sqrt(t303);
    let t3652 = t3651 * t3559;
    let t3654 = t1180 * t3587;
    let t3657 = t311 * t1379 * t313;
    let t3658 = 0.13692777777777777778e0 * t3657;
    let t3659 = t827 * t1187;
    let t3661 = t79 * t1311;
    let t3662 = t3661 * t3575;
    (t3640, t3641, t3643, t3646, t3651, t3652, t3654, t3657, t3658, t3659, t3661, t3662)
}
