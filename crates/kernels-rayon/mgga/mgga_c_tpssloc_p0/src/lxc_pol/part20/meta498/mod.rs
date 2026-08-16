//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta498 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2005;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2006;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta498(t12832: f64, t16505: f64, t3: f64, t112: f64, t5363: f64, t111: f64, t1851: f64, t2319: f64, t576: f64, t4072: f64, t671: f64, t1458: f64, t2363: f64, t12521: f64, t12524: f64, t12813: f64, t1401: f64, t3938: f64, t3941: f64, t5371: f64, t5376: f64, t577: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16506, t16507, t16521, t16524, t16535, t16538, t16541) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2005(t12832, t16505, t3, t112, t5363, t111, t1851, t2319, t576, t4072, t671, t1458, t2363);
        let t16546 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2006(t12521, t12524, t12813, t1401, t1458, t16506, t16521, t16524, t16535, t16538, t16541, t2319, t2363, t3938, t3941, t4072, t5371, t5376, t577, t671);
    (t16506, t16507, t16521, t16524, t16535, t16538, t16541, t16546)
}
