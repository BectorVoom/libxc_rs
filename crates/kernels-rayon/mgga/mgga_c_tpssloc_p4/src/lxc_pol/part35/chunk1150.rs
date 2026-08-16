//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1150/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1150(t23046: f64, t242: f64, t812: f64, t23062: f64, t7497: f64, t23077: f64, t6604: f64, t23083: f64, t7500: f64, t1509: f64, t236: f64, t23110: f64, t232: f64) -> (f64, f64, f64, f64, f64) {
    let t25083 = t23046 * t242;
    let t25084 = t812 * t25083;
    let t25109 = t23062 * t7497;
    let t25119 = t23077 * t6604;
    let t25126 = t23083 * t7500;
    let t25130 = t236 * t1509;
    let t25132 = t23110 * t25130 * t232;
    (t25084, t25109, t25119, t25126, t25132)
}
