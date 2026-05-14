//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1257/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1257<F: Float>(t15482: F, t20549: F, t35101: F, t1: F, t31740: F, t544: F, t10540: F, t18067: F, t2365: F, t25730: F, t4391: F, t25580: F, t10216: F, t10584: F, t1424: F, t1441: F, t1641: F, t31623: F, t31624: F, t3403: F, t3415: F, t35075: F, t35090: F, t35094: F, t35097: F, t35100: F, t4634: F, t4637: F, t531: F, t557: F, t568: F, t590: F, t597: F, t600: F) -> (F,) {
    let t35104 = 0.34082600847929555269e0 * t20549 * t15482 * t35101;
    let t35106 = t544 * t31740 * t1;
    let t35109 = t18067 * t10540;
    let t35110 = 0.59584149919750711116e-1 * t35109;
    let t35112 = t4391 * t2365 * t25730;
    let t35113 = 0.59584149919750711116e-1 * t35112;
    let t35115 = t4391 * t2365 * t25580;
    let t35116 = 0.29792074959875355558e-1 * t35115;
    let t35117 = 0.1022478025437886658e1 * t1441 * t10216 * t590 - t35075 - 0.35750489951850426669e0 * t557 * t531 * t31624 - 0.23005755572352449806e1 * t4634 * t3403 - 0.46011511144704899612e1 * t1641 * t10584 + 0.23005755572352449806e1 * t597 * t568 * t600 * t31623 + 0.23005755572352449806e1 * t4637 * t3415 - t35090 - t35094 - t35097 + t35100 - t35104 - 0.79445533226334281486e-1 * t35106 * t1424 + t35110 + t35113 + t35116;
    (t35117,)
}
