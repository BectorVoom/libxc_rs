//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1298/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1298<F: Float>(t11157: F, t832: F, t853: F, t1185: F, t27909: F, t3070: F, t9976: F, t11217: F, t11222: F, t11227: F, t11269: F, t11287: F, t1197: F, t18854: F, t18875: F, t2252: F, t22762: F, t22826: F, t27834: F, t31521: F, t31523: F, t31524: F, t31558: F, t31575: F, t3780: F, t3796: F, t6300: F, t6308: F, t863: F, t871: F, t891: F) -> (F, F, F, F) {
    let t31580 = t11157 * t832;
    let t31582 = F::new(1.0) * t31580 * t853;
    let t31584 = F::new(3.0) * t27909 * t1185;
    let t31586 = F::new(3.0) * t9976 * t3070;
    let t31587 = F::cast_from(0.35089341735807877242e1_f64) * t6300 * t11227 + t31521 - t31523 + F::cast_from(0.5848223622634646207e0_f64) * t31524 * t891 + F::cast_from(0.10254018858216406658e4_f64) * t18875 * t11217 - F::new(6.0) * t22826 * t3780 + F::new(6.0) * t6308 * t11222 + F::new(3.0) * t27834 * t1197 + F::cast_from(0.96491876992155210402e2_f64) * t22762 * t3796 - F::cast_from(0.19298375398431042081e3_f64) * t18854 * t11269 + F::new(1.0) * t2252 * t11287 + F::new(1.0) * t863 * (t31558 + t31575) * t871 - t31582 - t31584 - t31586;
    (t31582, t31584, t31586, t31587)
}
