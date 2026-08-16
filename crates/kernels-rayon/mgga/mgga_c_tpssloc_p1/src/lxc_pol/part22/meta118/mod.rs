//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta118 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk794;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk795;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk796;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk797;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk798;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk799;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk800;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk801;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta118(t3242: f64, t461: f64, t337: f64, t51: f64, t1887: f64, t1176: f64, t60: f64, t1184: f64, t1089: f64, t460: f64, t607: f64, t3247: f64, t3293: f64, t1191: f64, t225: f64, t1202: f64, t1226: f64, t3030: f64, t466: f64, t3032: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3441, t3447) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk794(t3242, t461, t337, t51, t1887);
        let t3448 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk795(t1176, t60);
        let t3449 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk796(t1184, t3448);
        let t3450 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk797(t1089, t460);
        let t3451 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk798(t3450, t607);
        let (t3455, t3464, t3487) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk799(t3247, t461, t3293, t1191, t225);
        let t3490 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk800(t1202, t1226);
        let (t3499, t3500) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk801(t3030, t466, t3032);
    (t3441, t3447, t3448, t3449, t3450, t3451, t3455, t3464, t3487, t3490, t3499, t3500)
}
