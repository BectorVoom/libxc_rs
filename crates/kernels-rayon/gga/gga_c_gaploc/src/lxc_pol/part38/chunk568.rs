//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 568/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk568(t4130: f64, t986: f64, t2482: f64, t9272: f64, t10231: f64, t549: f64, t544: f64, t8410: f64) -> (f64, f64, f64, f64, f64) {
    let t10608 = t4130 * t986;
    let t10609 = t10608 * t2482;
    let t10610 = t9272 * t10609;
    let t10611 = 0.57514388930881124514e0_f64 * t10610;
    let t10612 = t549 * t10231;
    let t10615 = t544 * t8410;
    (t10608, t10610, t10611, t10612, t10615)
}
