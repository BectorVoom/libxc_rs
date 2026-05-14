//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1402/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1402<F: Float>(t2368: F, t3730: F, t10088: F, t3185: F, t6475: F, t10093: F, t926: F, t394: F, t8309: F, t10054: F, t10075: F, t18979: F, t23319: F, t23325: F, t23331: F, t23338: F, t23340: F, t2380: F, t2381: F, t2396: F, t3186: F, t3206: F, t3898: F, t3919: F, t406: F, t6366: F, t6367: F, t6416: F, t6417: F, t8254: F, t8264: F, t8435: F, t8450: F, t8474: F) -> (F, F, F) {
    let t28256 = t3730 * t2368;
    let t28263 = t3185 * t6475 * t10088;
    let t28266 = t3185 * t926 * t10093;
    let t28272 = t394 * t8309;
    let t28281 = t23319 / 54.0 - t23325 / 144.0 - 2.0 / 81.0 * t23331 - 5.0 / 648.0 * t23338 + 0.51448821741683684368e-2 * t2380 * t8264 * t8474 + 0.17149607247227894789e-2 * t3206 * t8254 * t6416 * t10054 - 0.85748036236139473944e-3 * t8450 * t8254 * t18979 * t3898 + 0.42874018118069736972e-3 * t3206 * t2381 * t28256 * t2396 + 0.10162730220579493208e-2 * t23340 - 0.11433071498151929859e-2 * t28263 + 0.11433071498151929859e-2 * t28266 + 0.12862205435420921092e-2 * t2380 * t6366 * t3919 * t6367 - 0.42874018118069736972e-3 * t3206 * t406 * t3186 * t28272 - 0.12862205435420921092e-2 * t8435 * t406 * t10075 * t6417;
    (t28256, t28272, t28281)
}
