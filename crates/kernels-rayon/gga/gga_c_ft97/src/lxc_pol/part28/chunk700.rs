//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 700/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk700(t23890: f64, t23899: f64, t23914: f64, t23920: f64, t23924: f64, t27116: f64, t27121: f64, t27126: f64, t27130: f64, t27133: f64, t27135: f64, t27139: f64) -> f64 {
    let t27141 = -t27116 + t23890 / 6.0_f64 - t23899 - t23914 / 9.0_f64 + t23920 / 3.0_f64 - t27121 / 3.0_f64 + t27126 / 4.0_f64 + t27130 + t27133 - t27135 / 12.0_f64 - t23924 + t27139 / 6.0_f64;
    t27141
}
