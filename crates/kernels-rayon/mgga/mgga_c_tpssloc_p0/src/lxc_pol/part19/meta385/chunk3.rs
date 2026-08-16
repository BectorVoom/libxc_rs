//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1445/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1445(t1174: f64, t1186: f64, t2402: f64, t11498: f64, t135: f64, t457: f64, t625: f64, t221: f64, t456: f64, t461: f64, t11496: f64, t11569: f64, t11575: f64, t1184: f64, t15288: f64, t3447: f64, t3449: f64, t44415: f64, t44419: f64, t44424: f64, t44426: f64, t44432: f64, t44439: f64, t44445: f64, t44457: f64, t44470: f64, t460: f64, t4934: f64, t974: f64) -> (f64, f64) {
    let t44478 = t1174 * t2402 * t1186;
    let t44481 = t1174 * t135 * t11498;
    let t44483 = t625 * t457;
    let t44487 = 0.82304526748971193413e-3_f64 * t456 * t221 * t44483 * t461;
    let t44493 = 0.66666666666666666664e-2_f64 * t3447 * t3449 * t44415 - 0.44444444444444444444e-2_f64 * t3447 * t11569 * t44419 + 0.11111111111111111111e-2_f64 * t44424 - 0.83333333333333333332e-3_f64 * t1174 * t974 * t457 * t44426 * t460 - 0.24999999999999999999e-2_f64 * t1174 * t974 * t457 * t44432 * t460 + 0.11111111111111111111e-2_f64 * t44439 + 0.33333333333333333332e-2_f64 * t3447 * t11575 * t15288 - 0.11111111111111111111e-2_f64 * t44445 - 0.83333333333333333332e-3_f64 * t1174 * t974 * t457 * (t44457 + t44470) * t460 - 0.12345679012345679012e-2_f64 * t44478 - 0.11111111111111111111e-2_f64 * t44481 - t44487 - 0.33333333333333333332e-2_f64 * t1174 * t4934 * t11496 * t1184 * t460;
    (t44483, t44493)
}
