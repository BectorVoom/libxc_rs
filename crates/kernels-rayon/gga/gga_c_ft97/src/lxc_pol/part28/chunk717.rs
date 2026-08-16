//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 717/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk717(t23890: f64, t23914: f64, t23920: f64, t24034: f64, t24041: f64, t27116: f64, t27121: f64, t27126: f64, t27130: f64, t27133: f64, t27135: f64, t27139: f64) -> f64 {
    let t27376 = -t27116 / 3.0_f64 + t23890 / 18.0_f64 - t24034 - t23914 / 27.0_f64 + t23920 / 9.0_f64 - t27121 / 9.0_f64 + t27126 / 12.0_f64 + t27130 / 3.0_f64 + t27133 / 3.0_f64 - t27135 / 36.0_f64 - t24041 + t27139 / 18.0_f64;
    t27376
}
