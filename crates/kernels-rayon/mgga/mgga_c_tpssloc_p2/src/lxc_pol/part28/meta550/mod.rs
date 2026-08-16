//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta550 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1820;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta550(t111: f64, t7222: f64, t81437: f64, t112: f64, t24447: f64, t24007: f64, t22550: f64, t7031: f64, t22549: f64, t2031: f64, t83728: f64, t83737: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t84033, t84036, t84078, t84097, t84173, t84174, t84180, t84183) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1820(t111, t7222, t81437, t112, t24447, t24007, t22550, t7031, t22549, t2031, t83728, t83737);
    (t84033, t84036, t84078, t84097, t84173, t84174, t84180, t84183)
}
