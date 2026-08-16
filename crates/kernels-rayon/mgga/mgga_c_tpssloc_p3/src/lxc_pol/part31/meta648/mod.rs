//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta648 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1922;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1923;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta648(t16828: f64, t1888: f64, t6646: f64, t1484: f64, t1519: f64, t25038: f64, t25248: f64, t776: f64, t232: f64, t58262: f64, t23110: f64, t23185: f64, t28422: f64, t16817: f64, t82018: f64, t16825: f64, t22996: f64, t23168: f64, t28346: f64, t28338: f64, t81591: f64, t252: f64, t5544: f64, t22986: f64, t829: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98387, t98389, t98392, t98396, t98399) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1922(t16828, t1888, t6646, t1484, t1519, t25038, t25248, t776, t232, t58262, t23110, t23185, t28422);
        let (t98402, t98405, t98416, t98420, t98422, t98425) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1923(t16817, t1888, t82018, t16825, t22996, t23168, t28346, t28338, t81591, t252, t5544, t22986, t6646, t829);
    (t98387, t98389, t98392, t98396, t98399, t98402, t98405, t98416, t98420, t98422, t98425)
}
