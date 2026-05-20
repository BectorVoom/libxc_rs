//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2233/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2233<F: Float>(t2142: F, t6628: F, t3153: F, t11249: F, t5219: F, t7635: F, t6622: F, t73: F, t104482: F, t105383: F, t1203: F, t1248: F, t1287: F, t21472: F, t21582: F, t21586: F, t21595: F, t26889: F, t26895: F, t29159: F, t29175: F, t29194: F, t29195: F, t30739: F, t30751: F, t30840: F, t30853: F, t5458: F, t6744: F, t7627: F, t7636: F, t7637: F, t7651: F, t7652: F, t96929: F, t96953: F, t97041: F, t97308: F, t97313: F, t97314: F, t97318: F, t97319: F, t97397: F, t97398: F) -> (F, F, F, F, F) {
    let t111814 = t2142 * t6628;
    let t111815 = t111814 * t3153;
    let t111825 = t111814 * t11249;
    let t111832 = t5219 * t7635;
    let t111844 = t2142 * t6622;
    let t111845 = t111844 * t73;
    let t111864 = F::cast_from(0.8673628188205199462e0_f64) * t7651 * t7652 * t7627 * t6744 + F::cast_from(0.8673628188205199462e0_f64) * t97318 * t111815 * t97319 - F::cast_from(0.8673628188205199462e0_f64) * t97308 * t111815 * t21472 - F::cast_from(0.17347256376410398924e1_f64) * t29194 * t29195 * t21595 - F::cast_from(0.26020884564615598386e1_f64) * t104482 * t111825 * t21582 + F::cast_from(0.26020884564615598386e1_f64) * t105383 * t111825 * t21586 - F::cast_from(0.17347256376410398924e1_f64) * t111832 * t29175 + F::cast_from(0.34694512752820797848e1_f64) * t96953 * t30853 * t96929 - F::cast_from(0.8673628188205199462e0_f64) * t97397 * t111815 * t97398 + F::cast_from(0.17347256376410398924e1_f64) * t97313 * t111815 * t97314 - F::cast_from(0.8673628188205199462e0_f64) * t26889 * t111845 * t29159 + F::cast_from(0.8673628188205199462e0_f64) * t26895 * t111845 * t5458 - F::cast_from(0.26020884564615598386e1_f64) * t97041 * t30739 * t1248 * t1287 - F::cast_from(0.8673628188205199462e0_f64) * t26889 * t30751 * t1248 * t1287 - F::cast_from(0.8673628188205199462e0_f64) * t7636 * t7637 * t30840 * t1203;
    (t111815, t111825, t111844, t111845, t111864)
}
