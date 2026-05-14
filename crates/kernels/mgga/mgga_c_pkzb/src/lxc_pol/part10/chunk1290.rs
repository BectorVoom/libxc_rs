//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1290/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1290<F: Float>(t5713: F, t9605: F, t2038: F, t3656: F, t5939: F, t179: F, t299: F, t3515: F, t5672: F, t771: F, t9628: F, t9622: F, t11042: F, t17864: F, t18063: F, t18067: F, t18084: F, t18089: F, t2019: F, t2082: F, t21395: F, t25324: F, t25326: F, t2739: F, t2774: F, t2887: F, t295: F, t3647: F, t3662: F, t3666: F, t7725: F, t7768: F, t9568: F) -> (F,) {
    let t25553 = t5713 * t9605;
    let t25556 = t2038 * t5939 * t3656;
    let t25572 = t299 * t179 * t5672 * t3515;
    let t25576 = t771 * t9628;
    let t25580 = t771 * t9622;
    let t25584 = -t2887 * t21395 * t2774 * t2739 / 4.0 + 0.15244095330869239812e-2 * t25553 + 0.47637797908966374413e-4 * t25556 + 0.22866142996303859718e-2 * t7725 * t9568 + 0.10289764348336736873e-1 * t2019 * t295 * t25324 * t25326 * t11042 * t7768 - 5.0 / 648.0 * t18063 + t18067 / 432.0 - t18084 / 81.0 - t18089 / 216.0 + 0.95275595817932748827e-4 * t25572 + 0.43445671692977333466e-1 * t2082 * t3662 - 0.91464571985215438876e-2 * t25576 - 0.14481890564325777821e-1 * t2082 * t3666 + 0.30488190661738479624e-2 * t25580 - 0.28963781128651555643e-1 * t17864 * t3647;
    (t25584,)
}
