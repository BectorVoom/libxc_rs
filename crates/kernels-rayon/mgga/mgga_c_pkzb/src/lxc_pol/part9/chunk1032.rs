//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1032/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1032(t2381: f64, t8463: f64, t3188: f64, t926: f64, t3185: f64, t3224: f64, t6475: f64, t2380: f64, t3026: f64, t919: f64, t921: f64, t3206: f64, t3235: f64, t8408: f64, t8411: f64, t8415: f64, t8420: f64, t8424: f64, t8428: f64, t8432: f64, t8435: f64, t8438: f64, t8442: f64, t8447: f64, t8450: f64, t8453: f64, t8458: f64, t8460: f64) -> (f64, f64) {
    let t8464 = t2381 * t8463;
    let t8467 = t926 * t3188;
    let t8469 = 0.57165357490759649296e-3_f64 * t3185 * t8467;
    let t8470 = t6475 * t3224;
    let t8472 = 0.57165357490759649296e-3_f64 * t2380 * t8470;
    let t8473 = t3026 * t919;
    let t8474 = t8473 * t921;
    let t8475 = t2381 * t8474;
    let t8478 = t8408 + 0.25724410870841842184e-2_f64 * t3235 * t8411 + 0.12862205435420921092e-2_f64 * t3235 * t8415 - 0.51448821741683684368e-2_f64 * t3235 * t8420 + 0.42874018118069736972e-3_f64 * t3185 * t8424 + 0.12862205435420921092e-2_f64 * t8428 * t8432 - 0.12862205435420921092e-2_f64 * t8435 * t8438 - 0.42874018118069736972e-3_f64 * t3206 * t8442 - 0.21437009059034868486e-3_f64 * t3206 * t8447 + 0.21437009059034868486e-3_f64 * t8450 * t8453 - t8458 - 0.85748036236139473944e-3_f64 * t2380 * t8460 - 0.42874018118069736972e-3_f64 * t2380 * t8464 + t8469 - t8472 - 0.85748036236139473944e-3_f64 * t2380 * t8475;
    (t8474, t8478)
}
