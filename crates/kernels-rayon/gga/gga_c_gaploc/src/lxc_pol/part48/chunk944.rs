//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 944/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk944(t10805: f64, t8862: f64, t13346: f64, t4349: f64, t605: f64, t11135: f64, t10802: f64, t27229: f64, t11969: f64, t1960: f64, t977: f64, t24215: f64, t3553: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46008 = 4.0_f64 * t8862 * t10805;
    let t46011 = 12.0_f64 * t4349 * t13346 * t605;
    let t46013 = 4.0_f64 * t8862 * t11135;
    let t46016 = 12.0_f64 * t27229 * t10802;
    let t46019 = 2.0_f64 * t1960 * t11969 * t977;
    let t46023 = 2.0_f64 * t24215 * t3553;
    (t46008, t46011, t46013, t46016, t46019, t46023)
}
