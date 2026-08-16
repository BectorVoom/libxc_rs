//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta607 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2003;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2004;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta607(t1887: f64, t23069: f64, t22690: f64, t23153: f64, t23171: f64, t6561: f64, t80741: f64, t6643: f64, t23025: f64, t23030: f64, t23012: f64, t6653: f64, t22641: f64, t2588: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let t81591 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2003(t1887, t23069);
        let (t81595, t81597, t81599, t81600, t81602, t81612) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2004(t22690, t23153, t23171, t6561, t80741, t6643, t23025, t23030, t23012, t6653, t22641, t2588);
    (t81591, t81595, t81597, t81599, t81600, t81602, t81612)
}
