//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3141/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3141<F: Float>(t11570: F, t17686: F, t1174: F, t15269: F, t15274: F, t15288: F, t18420: F, t3447: F, t3449: F, t3469: F, t44487: F, t460: F, t4889: F, t4900: F, t4934: F, t6138: F, t64969: F, t64976: F, t64979: F, t64981: F, t64988: F, t64990: F) -> F {
    let t64994 = t11570 * t17686;
    let t65001 = -F::cast_from(0.55555555555555555554e-3_f64) * t64969 - F::cast_from(0.83333333333333333332e-3_f64) * t1174 * t4934 * t6138 * t3469 * t460 - F::cast_from(0.18106995884773662551e-2_f64) * t64976 + F::cast_from(0.6172839506172839506e-4_f64) * t64979 + F::cast_from(0.2962962962962962963e-2_f64) * t64981 + F::cast_from(0.88888888888888888888e-2_f64) * t4889 * t15269 + F::cast_from(0.44444444444444444444e-2_f64) * t4889 * t15274 - F::cast_from(0.18518518518518518518e-3_f64) * t64988 - t44487 + F::cast_from(0.88888888888888888886e-2_f64) * t3447 * t4900 * t64990 + F::cast_from(0.33333333333333333332e-2_f64) * t3447 * t3449 * t64994 + F::cast_from(0.55555555555555555554e-3_f64) * t3447 * t18420 * t15288;
    t65001
}
