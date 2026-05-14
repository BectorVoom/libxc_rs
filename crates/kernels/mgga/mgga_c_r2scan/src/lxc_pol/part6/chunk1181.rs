//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1181/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1181<F: Float>(t409: F, t5421: F, t1737: F, t18950: F, t18953: F, t640: F, t1399: F, t1691: F, t1818: F, t18956: F, t1945: F, t1986: F, t2006: F, t2017: F, t202: F, t207: F, t21247: F, t21257: F, t21262: F, t21264: F, t21268: F, t21270: F, t21519: F, t21529: F, t21531: F, t21533: F, t21535: F, t21540: F, t5270: F, t5317: F, t5549: F, t5568: F, t5589: F, t5642: F, t687: F, t713: F, t718: F) -> (F, F, F, F) {
    let t21542 = t5421 * t409;
    let t21544 = t1737 * t18950;
    let t21546 = t640 * t18953;
    let t21558 = 0.12865583598954028054e3 * t687 * t2017 * t5549 + 0.69263436422725855036e2 * t718 * t1986 * t5317 + t21247 + 0.57791679765211885293e1 * t21519 + 0.68445575878594514436e3 * t1818 * t1691 * t5568 - 0.56142946777292603589e2 * t1945 * t713 * t5270 + 1.0 * t202 * (0.10981366666666666667e3 * t21529 - 0.188252e3 * t21531 + 0.41833777777777777778e2 * t21533 - 0.48806074074074074073e2 * t21535 - 0.14025833333333333333e1 * t21540 + 0.134648e2 * t21542 - 0.49869629629629629631e1 * t21544 + 0.43635925925925925927e1 * t21546 + 0.10805407407407407408e1 * t18956) * t207 + 0.46316100956234500993e4 * t2006 * t2017 * t5589 - 0.26436201179130736843e2 * t1399 * t5642 - t21257 - t21262 - t21264 - t21268 + t21270;
    (t21542, t21544, t21546, t21558)
}
