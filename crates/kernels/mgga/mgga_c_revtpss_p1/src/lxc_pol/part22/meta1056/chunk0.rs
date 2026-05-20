//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3737/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3737<F: Float>(t12772: F, t17736: F, t21309: F, t3767: F, t70629: F, t474: F, t6593: F, t3089: F, t1285: F, t17384: F, t17605: F, t1012: F, t1222: F, t12256: F, t15936: F, t17658: F, t17729: F, t1774: F, t21030: F, t21121: F, t3631: F, t3692: F, t3720: F, t44225: F, t44484: F, t44609: F, t44664: F, t44675: F, t44696: F, t57786: F, t58777: F, t58831: F, t60717: F, t6688: F) -> (F, F) {
    let t70982 = t17736 * t12772 * t21309;
    let t70990 = t3767 * t70629;
    let t70993 = t474 * t6593;
    let t70994 = t70993 * t3089;
    let t70995 = t1285 * t70994;
    let t71009 = t17605 * t17384;
    let t71015 = -F::cast_from(0.76220476654346199061e-3_f64) * t70982 + F::cast_from(0.20325460441158986416e-2_f64) * t57786 - F::cast_from(0.95275595817932748826e-4_f64) * t44675 - t1222 * t1012 * t3692 * t60717 / F::new(72.0) + F::cast_from(0.60976381323476959248e-2_f64) * t70990 * t17658 - F::cast_from(0.96545937095505185476e-2_f64) * t70995 * t3631 + F::cast_from(0.1270341277572436651e-2_f64) * t17729 * t44225 * t1774 * t12256 * t15936 - F::cast_from(0.84689418504829110066e-4_f64) * t58777 - F::cast_from(0.52930886565518193792e-4_f64) * t44696 - F::cast_from(0.25724410870841842183e-2_f64) * t44609 * t3720 * t6688 * t58831 + F::cast_from(0.20325460441158986416e-2_f64) * t71009 - F::cast_from(0.17149607247227894789e-2_f64) * t44484 * t21121 + F::cast_from(0.85748036236139473944e-3_f64) * t44664 * t21030;
    (t70994, t71015)
}
