//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1188/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1188<F: Float>(t5318: F, t695: F, t182: F, t190: F, t2090: F, t625: F, t18956: F, t21529: F, t21531: F, t21533: F, t21535: F, t21540: F, t21542: F, t21544: F, t21546: F, t1399: F, t1719: F, t1821: F, t1838: F, t1917: F, t1923: F, t1966: F, t1981: F, t2030: F, t21311: F, t21416: F, t220: F, t225: F, t390: F, t5300: F, t5317: F, t5549: F, t5567: F, t5569: F, t5572: F, t5589: F, t5695: F, t5697: F, t5748: F, t5755: F, t5785: F, t5786: F, t681: F, t687: F, t690: F, t705: F, t712: F) -> (F, F, F, F) {
    let t21778 = t5318 * t695;
    let t21787 = 0.18467901234567901234e0 * t625 * t2090 * t182 * t190;
    let t21813 = 0.80554444444444444441e2 * t21529 - 0.13809333333333333333e3 * t21531 + 0.30687407407407407408e2 * t21533 - 0.3580197530864197531e2 * t21535 - 0.36677499999999999999e0 * t21540 + 0.352104e1 * t21542 - 0.13040888888888888888e1 * t21544 + 0.11410777777777777778e1 * t21546 + 0.8585111111111111111e0 * t18956;
    let t21820 = -0.77055573020282513724e1 * t1399 * t1838 - 0.23116671906084754117e2 * t390 * t5300 - 0.70178683471615754484e1 * t705 * t1917 * t1719 + 0.1929837539843104208e3 * t687 * t5755 * t1966 - 0.41558061853635513021e3 * t5572 * t21778 + 0.8276162067083744048e4 * t5785 * t5786 * t5549 - t21787 + 36.0 * t687 * t2030 * t1923 + 0.12304822629859687989e5 * t1981 * t712 * t5569 + 0.41016075432865626632e4 * t5567 * t1821 * t5317 * t695 + 0.37402255668271961718e4 * t5567 * t21311 + 0.79858241214418562928e6 * t5695 * t681 * t5697 * t5589 + 0.5848223622634646207e0 * t220 * t21813 * t225 - 0.7719350159372416832e4 * t5748 * t690 * t21416;
    (t21778, t21787, t21813, t21820)
}
