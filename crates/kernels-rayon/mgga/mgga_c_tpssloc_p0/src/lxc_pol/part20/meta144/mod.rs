//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta144 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk925;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk926;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk927;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk928;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk929;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta144(t3242: f64, t461: f64, t2244: f64, t3440: f64, t337: f64, t51: f64, t1887: f64, t1176: f64, t60: f64, t1184: f64, t1089: f64, t460: f64, t607: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3441, t3442, t3443, t3447) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk925(t3242, t461, t2244, t3440, t337, t51, t1887);
        let t3448 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk926(t1176, t60);
        let t3449 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk927(t1184, t3448);
        let t3450 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk928(t1089, t460);
        let t3451 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk929(t3450, t607);
    (t3441, t3442, t3443, t3447, t3448, t3449, t3450, t3451)
}
