//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta187 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1139;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1140;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1141;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta187(t4343: f64, t4518: f64, t3966: f64, t978: f64, t977: f64, t135: f64, t1599: f64, t973: f64, t1597: f64, t2987: f64, t2990: f64, t2824: f64, t3003: f64, t4384: f64, t4387: f64, t4390: f64, t4393: f64, t340: f64, t343: f64, t974: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4519, t4522, t4523, t4528, t4529, t4531) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1139(t4343, t4518, t3966, t978, t977, t135, t1599, t973, t1597, t2987);
        let (t4532, t4540) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1140(t2990, t4531, t2824, t3003, t4384, t4387, t4390, t4393);
        let (t4542, t4543, t4546) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1141(t340, t4540, t343, t974);
    (t4519, t4522, t4523, t4528, t4529, t4531, t4532, t4540, t4542, t4543, t4546)
}
