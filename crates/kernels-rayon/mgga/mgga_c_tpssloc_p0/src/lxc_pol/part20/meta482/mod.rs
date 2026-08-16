//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta482 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1966;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1967;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta482(t225: f64, t5213: f64, t1807: f64, t3879: f64, t5211: f64, t1332: f64, t5343: f64, t1372: f64, t1824: f64, t5250: f64, t5286: f64, t562: f64, t3851: f64, t5335: f64, t12248: f64, t68: f64, t544: f64, t12250: f64, t3791: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16022, t16028, t16030, t16033, t16036) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1966(t225, t5213, t1807, t3879, t5211, t1332, t5343, t1372, t1824);
        let (t16037, t16040, t16041, t16044, t16046, t16047, t16048) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1967(t16036, t5250, t5286, t562, t3851, t5335, t12248, t68, t544, t12250, t3791);
    (t16022, t16028, t16030, t16033, t16036, t16037, t16040, t16041, t16044, t16046, t16047, t16048)
}
