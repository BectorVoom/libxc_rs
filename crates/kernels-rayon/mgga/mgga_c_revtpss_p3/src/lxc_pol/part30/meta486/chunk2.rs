//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1829/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1829(t1398: f64, t543: f64, t7274: f64, t7301: f64, t2022: f64, t4056: f64, t3974: f64, t7259: f64, t2482: f64, t27: f64, t7269: f64) -> (f64, f64, f64, f64) {
    let t25960 = t7274 * t1398 * t543;
    let t25961 = t7301 * t25960;
    let t25965 = t2022 * t4056 * t543;
    let t25966 = t7301 * t25965;
    let t25969 = t7259 * t3974;
    let t25970 = 0.27104001498285508387e-3_f64 * t25969;
    let t25972 = t2482 * t7269 * t27;
    (t25961, t25966, t25970, t25972)
}
