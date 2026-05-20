//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1670/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1670<F: Float>(t44831: F, t487: F, t12657: F, t1269: F, t1204: F, t3727: F, t1210: F, t1211: F, t1215: F, t12600: F, t12607: F, t12621: F, t12628: F, t12629: F, t12633: F, t12641: F, t12647: F, t12666: F, t1274: F, t1277: F, t1294: F, t1295: F, t13166: F, t13182: F, t13184: F, t3561: F, t3572: F, t3576: F, t3584: F, t3585: F, t3732: F, t3737: F, t3738: F, t3790: F, t44778: F) -> F {
    let t45515 = t44831 * t487;
    let t45522 = t12657 * t1269;
    let t45535 = t1204 * t3727;
    let t45544 = -F::cast_from(0.79025390195226139183e1_f64) * t1210 * t3737 * t3584 * t3738 - F::cast_from(0.23707617058567841754e2_f64) * t12628 * t1211 * t44778 + F::cast_from(0.26341796731742046395e1_f64) * t1210 * t1277 * t12621 * t1294 + F::cast_from(0.15805078039045227836e2_f64) * t12628 * t1277 * t12629 * t1294 - F::cast_from(0.39512695097613069592e1_f64) * t12666 * t3585 - F::cast_from(0.15805078039045227836e2_f64) * t12641 * t12600 - F::cast_from(0.26341796731742046395e1_f64) * t45515 * t1215 + F::cast_from(0.79025390195226139183e1_f64) * t12666 * t3576 + F::cast_from(0.15805078039045227836e2_f64) * t12633 * t12647 - F::cast_from(0.79025390195226139183e1_f64) * t45522 * t1215 + F::cast_from(0.39512695097613069592e1_f64) * t1210 * t1277 * t3584 * t3790 + F::cast_from(0.15805078039045227836e2_f64) * t12641 * t12647 - F::cast_from(0.26341796731742046395e1_f64) * t3732 * t13166 + F::cast_from(0.79025390195226139183e1_f64) * t3572 * t12607 - F::cast_from(0.79025390195226139183e1_f64) * t45535 * t1295 - F::cast_from(0.23707617058567841754e2_f64) * t1274 * t13182 * t3738 * t3790 - F::cast_from(0.15805078039045227836e2_f64) * t3561 * t13184;
    t45544
}
