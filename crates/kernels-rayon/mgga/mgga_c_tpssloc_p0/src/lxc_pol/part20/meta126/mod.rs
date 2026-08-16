//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta126 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk824;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk825;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk826;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk827;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk828;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk829;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk830;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk831;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta126(t283: f64, t883: f64, t61: f64, t248: f64, t2771: f64, t363: f64, t368: f64, t1017: f64, t67: f64, t1058: f64, t1044: f64, t820: f64, t1023: f64, t884: f64, t225: f64, t3020: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3061 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk824(t283, t883);
        let (t3062, t3064) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk825(t3061, t61, t248, t2771);
        let (t3067, t3068) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk826(t363, t368, t1017, t67);
        let t3069 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk827(t3067, t3068);
        let t3070 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk828(t1058, t3069);
        let t3071 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk829(t1044, t820);
        let (t3072, t3073) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk830(t1023, t884, t3071);
        let t3076 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk831(t225, t3020);
    (t3061, t3062, t3064, t3067, t3068, t3069, t3070, t3071, t3072, t3073, t3076)
}
