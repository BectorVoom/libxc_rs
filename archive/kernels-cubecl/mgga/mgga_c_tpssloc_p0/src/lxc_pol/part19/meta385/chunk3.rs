//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1445/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1445<F: Float>(t1174: F, t1186: F, t2402: F, t11498: F, t135: F, t457: F, t625: F, t221: F, t456: F, t461: F, t11496: F, t11569: F, t11575: F, t1184: F, t15288: F, t3447: F, t3449: F, t44415: F, t44419: F, t44424: F, t44426: F, t44432: F, t44439: F, t44445: F, t44457: F, t44470: F, t460: F, t4934: F, t974: F) -> (F, F) {
    let t44478 = t1174 * t2402 * t1186;
    let t44481 = t1174 * t135 * t11498;
    let t44483 = t625 * t457;
    let t44487 = F::cast_from(0.82304526748971193413e-3_f64) * t456 * t221 * t44483 * t461;
    let t44493 = F::cast_from(0.66666666666666666664e-2_f64) * t3447 * t3449 * t44415 - F::cast_from(0.44444444444444444444e-2_f64) * t3447 * t11569 * t44419 + F::cast_from(0.11111111111111111111e-2_f64) * t44424 - F::cast_from(0.83333333333333333332e-3_f64) * t1174 * t974 * t457 * t44426 * t460 - F::cast_from(0.24999999999999999999e-2_f64) * t1174 * t974 * t457 * t44432 * t460 + F::cast_from(0.11111111111111111111e-2_f64) * t44439 + F::cast_from(0.33333333333333333332e-2_f64) * t3447 * t11575 * t15288 - F::cast_from(0.11111111111111111111e-2_f64) * t44445 - F::cast_from(0.83333333333333333332e-3_f64) * t1174 * t974 * t457 * (t44457 + t44470) * t460 - F::cast_from(0.12345679012345679012e-2_f64) * t44478 - F::cast_from(0.11111111111111111111e-2_f64) * t44481 - t44487 - F::cast_from(0.33333333333333333332e-2_f64) * t1174 * t4934 * t11496 * t1184 * t460;
    (t44483, t44493)
}
