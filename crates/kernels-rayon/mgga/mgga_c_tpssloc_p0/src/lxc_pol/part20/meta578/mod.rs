//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta578 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2142;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2143;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta578(t10870: f64, t3117: f64, t1020: f64, t10858: f64, t248: f64, t3101: f64, t10961: f64, t3108: f64, t10423: f64, t10937: f64, t2955: f64, t3158: f64, t10383: f64, t964: f64, t10508: f64, t3121: f64, t10949: f64, t11002: f64, t1036: f64, t10361: f64, t10390: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t43114, t43118, t43120, t43143, t43155) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2142(t10870, t3117, t1020, t10858, t248, t3101, t10961, t3108, t10423, t10937, t2955, t3158);
        let (t43157, t43161, t43167, t43176, t43186) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2143(t10383, t964, t1020, t10508, t248, t3121, t10949, t11002, t1036, t10361, t10390, t10423);
    (t43114, t43118, t43120, t43143, t43155, t43157, t43161, t43167, t43176, t43186)
}
