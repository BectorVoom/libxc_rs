//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1298/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1298(t11157: f64, t832: f64, t853: f64, t1185: f64, t27909: f64, t3070: f64, t9976: f64, t11217: f64, t11222: f64, t11227: f64, t11269: f64, t11287: f64, t1197: f64, t18854: f64, t18875: f64, t2252: f64, t22762: f64, t22826: f64, t27834: f64, t31521: f64, t31523: f64, t31524: f64, t31558: f64, t31575: f64, t3780: f64, t3796: f64, t6300: f64, t6308: f64, t863: f64, t871: f64, t891: f64) -> (f64, f64, f64, f64) {
    let t31580 = t11157 * t832;
    let t31582 = 1.0_f64 * t31580 * t853;
    let t31584 = 3.0_f64 * t27909 * t1185;
    let t31586 = 3.0_f64 * t9976 * t3070;
    let t31587 = 0.35089341735807877242e1_f64 * t6300 * t11227 + t31521 - t31523 + 0.5848223622634646207e0_f64 * t31524 * t891 + 0.10254018858216406658e4_f64 * t18875 * t11217 - 6.0_f64 * t22826 * t3780 + 6.0_f64 * t6308 * t11222 + 3.0_f64 * t27834 * t1197 + 0.96491876992155210402e2_f64 * t22762 * t3796 - 0.19298375398431042081e3_f64 * t18854 * t11269 + 1.0_f64 * t2252 * t11287 + 1.0_f64 * t863 * (t31558 + t31575) * t871 - t31582 - t31584 - t31586;
    (t31582, t31584, t31586, t31587)
}
