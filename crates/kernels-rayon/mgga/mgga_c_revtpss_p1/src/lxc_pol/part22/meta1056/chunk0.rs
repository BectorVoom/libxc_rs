//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3737/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3737(t12772: f64, t17736: f64, t21309: f64, t3767: f64, t70629: f64, t474: f64, t6593: f64, t3089: f64, t1285: f64, t17384: f64, t17605: f64, t1012: f64, t1222: f64, t12256: f64, t15936: f64, t17658: f64, t17729: f64, t1774: f64, t21030: f64, t21121: f64, t3631: f64, t3692: f64, t3720: f64, t44225: f64, t44484: f64, t44609: f64, t44664: f64, t44675: f64, t44696: f64, t57786: f64, t58777: f64, t58831: f64, t60717: f64, t6688: f64) -> (f64, f64) {
    let t70982 = t17736 * t12772 * t21309;
    let t70990 = t3767 * t70629;
    let t70993 = t474 * t6593;
    let t70994 = t70993 * t3089;
    let t70995 = t1285 * t70994;
    let t71009 = t17605 * t17384;
    let t71015 = -0.76220476654346199061e-3_f64 * t70982 + 0.20325460441158986416e-2_f64 * t57786 - 0.95275595817932748826e-4_f64 * t44675 - t1222 * t1012 * t3692 * t60717 / 72.0_f64 + 0.60976381323476959248e-2_f64 * t70990 * t17658 - 0.96545937095505185476e-2_f64 * t70995 * t3631 + 0.1270341277572436651e-2_f64 * t17729 * t44225 * t1774 * t12256 * t15936 - 0.84689418504829110066e-4_f64 * t58777 - 0.52930886565518193792e-4_f64 * t44696 - 0.25724410870841842183e-2_f64 * t44609 * t3720 * t6688 * t58831 + 0.20325460441158986416e-2_f64 * t71009 - 0.17149607247227894789e-2_f64 * t44484 * t21121 + 0.85748036236139473944e-3_f64 * t44664 * t21030;
    (t70994, t71015)
}
