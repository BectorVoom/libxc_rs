//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 901/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk901(t16616: f64, t763: f64, t2752: f64, t5664: f64, t4101: f64, t4205: f64, t5575: f64, t68: f64) -> (f64, f64, f64, f64) {
    let t16617 = t16616 * t763;
    let t16625 = t5664 * t2752;
    let t16630 = t4205 * t4101;
    let t16673 = t5575 * t68;
    (t16617, t16625, t16630, t16673)
}
