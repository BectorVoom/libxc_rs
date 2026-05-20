//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2913/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2913<F: Float>(t19056: F, t4632: F, t19327: F, t52645: F, t15416: F, t6142: F, t52505: F, t6110: F, t11450: F, t11461: F, t15241: F, t19272: F, t19276: F, t19304: F, t23711: F, t23714: F, t23785: F, t2982: F, t41788: F, t52440: F, t52511: F, t52637: F, t52837: F, t52840: F, t6173: F, t6190: F, t6209: F, t77612: F, t77622: F, t953: F) -> (F, F, F, F, F) {
    let t77624 = F::new(3.0) * t19056 * t4632;
    let t77628 = F::new(18.0) * t52645 * t19327;
    let t77634 = F::new(3.0) * t15416 * t6142;
    let t77636 = F::new(6.0) * t52505 * t6110;
    let t77637 = F::new(18.0) * t52840 * t19272 + F::cast_from(0.62071215503128080361e4_f64) * t11450 * t6173 * t15241 * t953 + t77612 - F::cast_from(0.35089341735807877242e1_f64) * t52440 * t6190 + F::cast_from(0.35089341735807877242e1_f64) * t11461 * t23711 + F::cast_from(0.5848223622634646207e0_f64) * t2982 * t23714 - F::cast_from(0.31168546390226634766e3_f64) * t52511 * t19304 - t77622 - t77624 - F::cast_from(0.57895126195293126241e3_f64) * t52837 * t19276 - t77628 + F::cast_from(0.51947577317044391276e2_f64) * t52637 * t6209 - F::cast_from(0.10389515463408878255e3_f64) * t41788 * t23785 - t77634 + t77636;
    (t77624, t77628, t77634, t77636, t77637)
}
