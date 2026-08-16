//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3755/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3755(t3717: f64, t70994: f64, t1122: f64, t1250: f64, t12866: f64, t1715: f64, t17353: f64, t17539: f64, t17651: f64, t17661: f64, t17673: f64, t17736: f64, t17737: f64, t17800: f64, t20721: f64, t20959: f64, t3568: f64, t3626: f64, t3723: f64, t44521: f64, t44925: f64, t44931: f64, t57631: f64, t57663: f64, t59142: f64, t59144: f64, t59146: f64, t70496: f64) -> f64 {
    let t71513 = t3717 * t70994;
    let t71527 = t44925 / 1296.0_f64 + 0.28582678745379824648e-3_f64 * t12866 * t17661 * t17673 - 5.0_f64 / 1944.0_f64 * t44931 + 0.25724410870841842184e-2_f64 * t57631 * t20959 - 0.11433071498151929859e-2_f64 * t17736 * t3626 * t20721 * t1122 - 0.57165357490759649296e-3_f64 * t17736 * t3626 * t17737 * t17539 - 0.57165357490759649296e-3_f64 * t59142 - 0.14481890564325777821e-1_f64 * t71513 * t3723 - 5.0_f64 / 243.0_f64 * t59144 + 0.57165357490759649296e-3_f64 * t57663 * t17651 - 0.11433071498151929859e-2_f64 * t70496 * t17800 - 0.57165357490759649296e-3_f64 * t44521 * t17353 * t1250 * t1715 * t3568 - 0.57165357490759649296e-3_f64 * t59146;
    t71527
}
