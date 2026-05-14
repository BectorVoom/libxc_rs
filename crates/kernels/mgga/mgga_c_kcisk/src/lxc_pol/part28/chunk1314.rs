//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1314/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1314<F: Float>(t10879: F, t9740: F, t9742: F, t48397: F, t79: F, t2803: F, t33176: F, t9732: F, t5439: F, t5507: F, t18681: F, t2804: F, t2806: F, t12261: F, t9727: F, t33276: F, t9721: F) -> (F, F, F, F, F, F, F, F) {
    let t112925 = t9740 * t10879 * t9742;
    let t112933 = t48397 * t79;
    let t112934 = t112933 * t2803;
    let t112937 = t33176 * t9732;
    let t112982 = t5507 * t5439;
    let t113003 = 0.19290123456790123457e-2 * t2804 * t18681 * t2806;
    let t113037 = t12261 * t9727;
    let t113038 = t2804 * t113037;
    let t113042 = t9721 * t33276;
    (t112925, t112934, t112937, t112982, t113003, t113037, t113038, t113042)
}
