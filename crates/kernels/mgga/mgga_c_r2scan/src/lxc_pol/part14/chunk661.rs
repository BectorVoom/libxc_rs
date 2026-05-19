//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 661/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk661<F: Float>(t1380: F, t377: F, t124: F, t446: F, t1428: F, t1434: F, t1445: F, t1446: F, t1451: F, t1454: F, t1498: F, t1505: F, t1506: F, t432: F, t437: F, t439: F, t454: F, t4703: F, t4705: F, t4708: F, t4711: F, t4715: F, t4721: F, t4727: F, t4729: F, t4754: F, t4759: F, t4762: F, t4768: F, t4772: F, t5: F, t625: F, t72: F, t85: F) -> F {
    let t4776 = t377 * t1380;
    let t4780 = t124 * t446;
    let t4784 = -F::new(6.0) * t1434 * t439 * t1445 - t4703 + F::cast_from(0.51947577317044391277e2_f64) * t1505 * t4705 - F::cast_from(0.35089341735807877242e1_f64) * t1498 * t4708 + F::cast_from(0.96491876992155210402e2_f64) * t1451 * t4711 * t437 + F::cast_from(0.56968947174242584612e-3_f64) * t5 * t4715 * t85 - t4721 + F::cast_from(0.16562821945185185185e-2_f64) * t5 * t4715 * t72 - F::cast_from(0.19298375398431042081e3_f64) * t4727 * t4729 + F::new(1.0) * t432 * t4754 + F::cast_from(0.2069040516770936012e4_f64) * t4759 * t4762 - F::cast_from(0.51369999999999999999e-1_f64) * t625 * t1428 * t1446 - F::cast_from(0.16522625736956710527e1_f64) * t625 * t4768 * t1454 + F::cast_from(0.68493333333333333332e-1_f64) * t625 * t4772 * t439 - F::cast_from(0.48159733137676571078e0_f64) * t625 * t4776 * t1506 + F::cast_from(0.21687162600603479684e-1_f64) * t625 * t4780 * t454;
    t4784
}
