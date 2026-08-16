//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 662/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk662(t1380: f64, t377: f64, t124: f64, t446: f64, t1428: f64, t1434: f64, t1445: f64, t1446: f64, t1451: f64, t1454: f64, t1498: f64, t1505: f64, t1506: f64, t432: f64, t437: f64, t439: f64, t454: f64, t4703: f64, t4705: f64, t4708: f64, t4711: f64, t4715: f64, t4721: f64, t4727: f64, t4729: f64, t4754: f64, t4759: f64, t4762: f64, t4768: f64, t4772: f64, t5: f64, t625: f64, t72: f64, t85: f64) -> f64 {
    let t4776 = t377 * t1380;
    let t4780 = t124 * t446;
    let t4784 = -6.0_f64 * t1434 * t439 * t1445 - t4703 + 0.51947577317044391277e2_f64 * t1505 * t4705 - 0.35089341735807877242e1_f64 * t1498 * t4708 + 0.96491876992155210402e2_f64 * t1451 * t4711 * t437 + 0.56968947174242584612e-3_f64 * t5 * t4715 * t85 - t4721 + 0.16562821945185185185e-2_f64 * t5 * t4715 * t72 - 0.19298375398431042081e3_f64 * t4727 * t4729 + 1.0_f64 * t432 * t4754 + 0.2069040516770936012e4_f64 * t4759 * t4762 - 0.51369999999999999999e-1_f64 * t625 * t1428 * t1446 - 0.16522625736956710527e1_f64 * t625 * t4768 * t1454 + 0.68493333333333333332e-1_f64 * t625 * t4772 * t439 - 0.48159733137676571078e0_f64 * t625 * t4776 * t1506 + 0.21687162600603479684e-1_f64 * t625 * t4780 * t454;
    t4784
}
