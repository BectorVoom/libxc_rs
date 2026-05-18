//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 484/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk484<F: Float>(t1643: F, t573: F, t586: F, t4636: F, t1675: F, t596: F, t4722: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t4740 = t1643 * t1643;
    let t4741 = F::new(1.0) / t4740;
    let t4742 = t573 * t4741;
    let t4743 = t586 * t586;
    let t4744 = F::new(1.0) / t4743;
    let t4748 = F::new(0.12361111111111111111e-1) * t4636;
    let t4760 = t1675 * t596;
    let t4761 = F::new(1.0) / t4760;
    let t4769 = F::new(0.40256666666666666667e0) * t4636;
    let t4776 = F::new(0.137975e0) * t4722;
    let t4786 = t1675 * t1675;
    let t4787 = F::new(1.0) / t4786;
    (t4740, t4741, t4742, t4743, t4744, t4748, t4761, t4769, t4776, t4786, t4787)
}
