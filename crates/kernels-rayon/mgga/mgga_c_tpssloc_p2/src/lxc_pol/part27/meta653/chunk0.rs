//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2280/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2280(t24996: f64, t90442: f64, t24995: f64, t34475: f64, t5308: f64, t1983: f64, t26503: f64, t6999: f64, t12823: f64, t7468: f64, t26003: f64, t4034: f64) -> (f64, f64, f64, f64, f64) {
    let t90444 = 12.0_f64 * t90442 * t24996;
    let t90447 = 12.0_f64 * t24995 * t34475 * t5308;
    let t90450 = 2.0_f64 * t1983 * t26503 * t6999;
    let t90454 = 2.0_f64 * t12823 * t7468;
    let t90456 = 4.0_f64 * t4034 * t26003;
    (t90444, t90447, t90450, t90454, t90456)
}
