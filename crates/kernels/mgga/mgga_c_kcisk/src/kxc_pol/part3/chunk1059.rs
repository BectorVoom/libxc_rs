//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 1059/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk1059<F: Float>(t15762: F, t233: F, t2053: F, t4574: F, t564: F, t1149: F, t3299: F, t1625: F, t3465: F, t222: F, t3276: F, t1056: F, t3277: F) -> (F, F, F, F, F, F) {
    let t15763 = t233 * t15762;
    let t15764 = t4574 * t2053;
    let t15765 = t564 * t15764;
    let t15766 = F::new(3.0) / F::new(16.0) * t15765;
    let t15767 = t3299 * t1149;
    let t15769 = t3465 * t1625;
    let t15770 = F::new(3.0) / F::new(8.0) * t15769;
    let t15772 = F::new(1.0) / t3276 / t222;
    let t15775 = t3277 * t1056;
    (t15763, t15766, t15767, t15770, t15772, t15775)
}
