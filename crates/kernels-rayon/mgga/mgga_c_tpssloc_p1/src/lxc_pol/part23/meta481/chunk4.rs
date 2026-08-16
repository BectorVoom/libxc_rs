//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1443/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1443(t1174: f64, t1177: f64, t1714: f64, t18321: f64, t22032: f64, t22047: f64, t22052: f64, t22082: f64, t3440: f64, t3441: f64, t3455: f64, t44487: f64, t44621: f64, t44622: f64, t460: f64, t4889: f64, t4934: f64, t6120: f64, t65002: f64, t65023: f64, t73491: f64, t75836: f64, t75847: f64) -> f64 {
    let t78545 = -t44487 - 0.19753086419753086419e-2_f64 * t65002 + 0.92181069958847736624e-2_f64 * t4889 * t22082 + 0.28806584362139917695e-2_f64 * t1174 * t44621 * t44622 * t75836 - 0.59259259259259259257e-2_f64 * t65023 + 0.14814814814814814815e-2_f64 * t73491 - 0.33333333333333333332e-2_f64 * t1174 * t4934 * t22032 * t1714 * t460 + 0.11111111111111111111e-2_f64 * t1174 * t3440 * t3441 * t75847 - 0.16666666666666666666e-2_f64 * t1174 * t1177 * t3455 * t75847 + 0.21728395061728395061e-1_f64 * t18321 * t6120 + 0.26666666666666666666e-1_f64 * t4889 * t22052 + 0.29629629629629629628e-2_f64 * t4889 * t22047;
    t78545
}
