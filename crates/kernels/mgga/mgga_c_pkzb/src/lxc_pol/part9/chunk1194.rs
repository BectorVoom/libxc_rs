//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1194/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1194<F: Float>(t19109: F, t6460: F, t19116: F, t22971: F, t6517: F, t3185: F, t6475: F, t8350: F, t3206: F, t8354: F, t10213: F, t1249: F, t18661: F, t18662: F, t18974: F, t18987: F, t18992: F, t18997: F, t22957: F, t22973: F, t23020: F, t23022: F, t23028: F, t23054: F, t23055: F, t23061: F, t23075: F, t2380: F, t2381: F, t2382: F, t3186: F, t3265: F, t394: F, t406: F, t6106: F, t6367: F, t6416: F, t6483: F, t6518: F, t6526: F, t8380: F, t8428: F, t8430: F, t8435: F, t8445: F, t8532: F) -> (F, F, F) {
    let t23076 = t19109 * t6460;
    let t23081 = t19116 * t22971;
    let t23082 = t6517 * t6460;
    let t23088 = t3185 * t6475 * t8350;
    let t23091 = t3206 * t6475 * t8354;
    let t23093 = t23020 / 24.0 + 0.25724410870841842183e-2 * t8435 * t2381 * t23022 * t6526 - 0.25724410870841842184e-2 * t23028 - 0.12862205435420921092e-2 * t2380 * t2381 * t8532 * t2382 - t18987 / 288.0 - t18992 / 72.0 - t18997 / 48.0 - 0.38586616306262763275e-2 * t3206 * t10213 * t6416 * t6367 - 0.12862205435420921092e-2 * t2380 * t2381 * t3265 * t6483 - 0.42874018118069736972e-3 * t2380 * t2381 * t1249 * t394 * t6106 - 0.64311027177104605458e-3 * t3206 * t406 * t8380 * t8445 + 0.38586616306262763276e-2 * t8428 * t23054 * t6518 * t23055 + 0.25724410870841842184e-2 * t23061 - 0.51448821741683684368e-2 * t2380 * t18661 * t1249 * t18662 + 0.38586616306262763275e-2 * t8428 * t406 * t22957 * t8430 + 0.42874018118069736972e-3 * t3185 * t406 * t3186 * t18974 + 0.51448821741683684368e-2 * t23075 * t406 * t22973 * t23076 - 0.77173232612525526552e-2 * t23081 * t406 * t22973 * t23082 - 0.17149607247227894789e-2 * t23088 + 0.85748036236139473944e-3 * t23091;
    (t23076, t23082, t23093)
}
