//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1361/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1361(t2018: f64, t22574: f64, t24432: f64, t5187: f64, t24995: f64, t37790: f64, t5308: f64, t2314: f64, t33617: f64, t4034: f64, t652: f64, t7156: f64, t7467: f64) -> (f64, f64, f64, f64, f64) {
    let t120986 = 3.0_f64 * t22574 * t24432 * t2018 * t5187;
    let t120991 = 6.0_f64 * t24995 * t37790 * t5308;
    let t120993 = 2.0_f64 * t2314 * t33617;
    let t120995 = 2.0_f64 * t4034 * t33617;
    let t120998 = 2.0_f64 * t652 * t7156 * t7467;
    (t120986, t120991, t120993, t120995, t120998)
}
