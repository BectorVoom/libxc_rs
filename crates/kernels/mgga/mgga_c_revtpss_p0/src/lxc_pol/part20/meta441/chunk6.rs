//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1681/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1681<F: Float>(t1210: F, t1211: F, t1214: F, t1215: F, t12607: F, t12622: F, t12658: F, t12696: F, t1274: F, t1277: F, t1294: F, t1295: F, t13165: F, t13166: F, t13177: F, t3556: F, t3561: F, t3567: F, t3568: F, t3576: F, t3585: F, t3737: F, t3738: F, t44944: F, t45391: F, t45545: F, t45552: F, t45553: F, t45559: F, t45568: F, t45575: F, t45617: F, t45652: F, t45691: F, t45723: F, t45760: F, t45800: F, t45838: F, t45873: F) -> F {
    let t45895 = -F::cast_from(0.79025390195226139183e1_f64) * t45545 * t1215 - F::cast_from(0.26341796731742046395e1_f64) * t3556 * t12622 + F::cast_from(0.15805078039045227836e2_f64) * t1274 * t45552 * t45553 + F::cast_from(0.79025390195226139183e1_f64) * t3556 * t12607 + F::cast_from(0.39512695097613069591e1_f64) * t1274 * t3737 * t45559 + F::cast_from(0.52683593463484092788e1_f64) * t3567 * t1211 * t44944 + F::cast_from(0.15805078039045227836e2_f64) * t3561 * t12696 - F::cast_from(0.26341796731742046395e1_f64) * t45568 * t1215 + F::cast_from(0.26341796731742046395e1_f64) * t1210 * t1277 * t13165 * t1214 - F::cast_from(0.26341796731742046395e1_f64) * t45575 * t1295 - F::cast_from(0.79025390195226139183e1_f64) * t13177 * t3585 - F::cast_from(0.65854491829355115987e0_f64) * t1274 * t1277 * (t45617 + t45652 + t45691 + t45723 + t45760 + t45800 + t45838 + t45873) + F::cast_from(0.15805078039045227836e2_f64) * t3567 * t3737 * t3568 * t3738 + F::cast_from(0.79025390195226139183e1_f64) * t12658 * t3576 - F::cast_from(0.26341796731742046395e1_f64) * t3561 * t13166 + F::cast_from(0.39512695097613069591e1_f64) * t3567 * t1211 * t45391 + F::cast_from(0.52683593463484092788e1_f64) * t1274 * t3737 * t13165 * t1294;
    t45895
}
