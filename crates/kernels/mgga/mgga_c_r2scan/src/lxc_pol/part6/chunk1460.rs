//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1460/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1460<F: Float>(t19712: F, t20180: F, t25039: F, t25041: F, t25043: F, t25045: F, t27397: F, t27400: F, t27404: F, t27408: F, t27412: F, t7155: F, t18786: F, t18839: F, t18843: F, t18855: F, t23258: F, t23263: F, t23320: F, t23321: F, t23682: F, t23685: F, t23688: F, t23695: F, t23703: F, t23717: F, t23725: F, t23749: F, t23766: F, t23782: F, t23804: F, t23832: F, t23913: F, t23936: F, t23944: F, t23971: F, t23993: F, t27394: F, t27413: F, t27420: F, t27424: F, t27428: F, t27431: F, t27432: F, t27434: F, t27435: F, t27439: F, t27440: F, t27442: F, t27446: F, t27449: F, t27450: F, t27454: F, t354: F, t374: F, t5027: F, t5029: F, t5032: F, t5034: F, t6900: F, t6903: F, t8: F, t8302: F, t9922: F, t9923: F) -> (F,) {
    let t27455 = t27397 + t25039 - t25041 + t27400 - t19712 + t27404 + t27408 + t27412 - t25043 - t20180 - t25045;
    let t27461 = 0.35089341735807877242e1 * t7155;
    let t27464 = -72.0 * t5027 - 0.52634012603711815863e1 * t5029 + t23258 + 3.0 * t8302 + t8 * (t354 * (t23695 + t23703 + t23717 + t23725 + t23749 + t23766 + t23782 + t23804 + t23832 + t23913 + t23936 + t23944 + t23971 + t23993 + t27394 + t27413) + t23682 * t374 + t18843 + t27420 - t18855 + t27454 + t27455 + t27439 + t27440 + t27434 + t27435 + t27446 - t18786 + t27424 + t27431 + t27432 + t27442 - t23688 - t23685 + t27450 - t18839 + t23320 + t23321 + t27449 + t27428) + t6900 - t27461 - t9922 - t9923 + 6.0 * t5032 - t6903 + 0.10526802520742363173e2 * t5034 + t23263;
    (t27464,)
}
