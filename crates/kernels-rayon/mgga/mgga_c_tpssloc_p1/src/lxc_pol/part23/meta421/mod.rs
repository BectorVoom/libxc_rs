//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta421 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1246;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1247;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta421(t21444: f64, t2987: f64, t13784: f64, t21122: f64, t2986: f64, t21456: f64, t20217: f64, t2989: f64, t20234: f64, t43070: f64, t10236: f64, t135: f64, t21458: f64, t973: f64, t42841: f64, t4514: f64, t61189: f64, t21446: f64, t21510: f64, t13779: f64, t21126: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t69496, t69503, t69505, t69515, t69519, t69529, t69540) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1246(t21444, t2987, t13784, t21122, t2986, t21456, t20217, t2989, t20234, t43070, t10236, t135, t21458, t973);
        let (t69548, t69570, t69579, t69647, t69683) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1247(t20234, t42841, t2986, t4514, t61189, t135, t21446, t973, t10236, t21510, t13779, t21126);
    (t69496, t69503, t69505, t69515, t69519, t69529, t69540, t69548, t69570, t69579, t69647, t69683)
}
