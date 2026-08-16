//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta398 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1633;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1634;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1635;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta398(t15437: f64, t3514: f64, t3572: f64, t5002: f64, t3523: f64, t5005: f64, t5019: f64, t5024: f64, t11147: f64, t11778: f64, t14165: f64, t4582: f64, t1735: f64, t3252: f64, t3578: f64, t3248: f64, t11642: f64, t11644: f64, t11649: f64, t1174: f64, t1227: f64, t15434: f64, t3518: f64, t3527: f64, t3531: f64, t3577: f64, t1216: f64, t4733: f64, t1653: f64, t3494: f64, t1090: f64, t5012: f64, t3490: f64, t4993: f64, t248: f64, t3521: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15438, t15446, t15448, t15450, t15452, t15455) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1633(t15437, t3514, t3572, t5002, t3523, t5005, t5019, t5024, t11147, t11778, t14165, t4582);
        let (t15459, t15463, t15466) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1634(t1735, t3252, t3578, t3248, t11642, t11644, t11649, t1174, t1227, t15434, t15438, t15446, t15448, t15450, t15452, t15455, t3518, t3527, t3531, t3577, t5005);
        let (t15470, t15474, t15478, t15484, t15486, t15488) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1635(t1216, t4733, t3578, t1653, t3494, t1090, t5012, t3490, t4993, t248, t3521, t1227);
    (t15455, t15459, t15463, t15466, t15470, t15474, t15478, t15484, t15486, t15488)
}
