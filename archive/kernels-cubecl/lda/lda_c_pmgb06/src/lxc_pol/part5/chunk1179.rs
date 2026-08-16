//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1179/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1179<F: Float>(t21240: F, t21254: F, t10886: F, t117: F, t123: F, t125: F, t14527: F, t14529: F, t14536: F, t14541: F, t14544: F, t14545: F, t14550: F, t18054: F, t18059: F, t18062: F, t18064: F, t18069: F, t18071: F, t18076: F, t19205: F, t19222: F, t19248: F, t19277: F, t19303: F, t19429: F, t19452: F, t19483: F, t19511: F, t19531: F, t19544: F, t19596: F, t19640: F, t19674: F, t19695: F, t19710: F, t19734: F, t19751: F, t19950: F, t19973: F, t19984: F, t20003: F, t20040: F, t20068: F, t20105: F, t20124: F, t20145: F, t20178: F, t20208: F, t20227: F, t20261: F, t20289: F, t20322: F, t20347: F, t20379: F, t20416: F, t20450: F, t20473: F, t20500: F, t20513: F, t20530: F, t20564: F, t20597: F, t20623: F, t20661: F, t20678: F, t20753: F, t20781: F, t20806: F, t20837: F, t20861: F, t20885: F, t20904: F, t20915: F, t20947: F, t20978: F, t21010: F, t21045: F, t21067: F, t21089: F) -> (F, F) {
    let t21255 = t21240 + t21254;
    let t21267 = -F::cast_from(0.1890324433388467_f64) * t18054 - t10886 - F::cast_from(0.0014862827083471494_f64) * t18059 + F::cast_from(0.09451622166942335_f64) * t18062 + F::cast_from(0.1890324433388467_f64) * t18064 - F::cast_from(0.09451622166942335_f64) * t18069 - F::cast_from(0.09451622166942335_f64) * t18071 + F::cast_from(0.2634331482256014_f64) * t14527 + F::cast_from(0.02694202652307287_f64) * t18076 - F::cast_from(0.09451622166942335_f64) * t14529 - t14536 - F::cast_from(0.005388405304614574_f64) * t123 * t125 * (t20753 + t20145 + t19222 + t20068 + t19973 + t20289 + t19751 + t20416 + t20978 + t20623 + t20837 + t20885 + t21255 + t20904 + t20379 + t20178 + t19531 + t19483 + t20947 + t20661 + t20347 + t19511 + t20781 + t20513 + t19950 + t20500 + t19452 + t20227 + t19544 + t19429 + t20564 + t21089 + t20473 + t20678 + t19710 + t20208 + t20322 + t20530 + t20597 + t19734 + t20861 + t20003 + t19640 + t20105 + t21010 + t20040 + t19674 + t19277 + t20806 + t19984 + t20450 + t19205 + t20261 + t21045 + t20124 + t20915 + t19248 + t19695 + t19303 + t21067 + t19596) * t117 + F::cast_from(0.2835486650082701_f64) * t14541 - t14544 - F::cast_from(0.5670973300165402_f64) * t14545 + t14550;
    (t21255, t21267)
}
