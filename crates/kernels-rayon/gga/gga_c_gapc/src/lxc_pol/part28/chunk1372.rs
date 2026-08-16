//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1372/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1372(t33289: f64, t33292: f64, t33295: f64, t33298: f64, t33301: f64, t33305: f64, t33313: f64, t33315: f64, t33320: f64, t33324: f64, t33326: f64, t33330: f64, t33333: f64, t33336: f64, t33339: f64, t33341: f64, t33343: f64, t33346: f64, t33349: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t36535 = 0.63350674672043801542e-5_f64 * t33289;
    let t36536 = 0.2318836277704281739e-4_f64 * t33292;
    let t36537 = 0.43440462632258606772e-4_f64 * t33295;
    let t36538 = 0.43440462632258606772e-4_f64 * t33298;
    let t36539 = 0.21720231316129303386e-4_f64 * t33301;
    let t36540 = 0.17632363114482012216e-5_f64 * t33305;
    let t36542 = 0.1371666545474996961e-6_f64 * t33313;
    let t36543 = 0.3243554543208642639e-2_f64 * t33315;
    let t36556 = 0.43440462632258606772e-4_f64 * t33320 - 0.69504740211613770836e-3_f64 * t33324 - 0.3243554543208642639e-2_f64 * t33326 + 0.1433927048577202691e-8_f64 * t33330 - 0.2318836277704281739e-4_f64 * t33333 - 0.12290803273518880209e-8_f64 * t33336 + 0.16387737698025173612e-8_f64 * t33339 + 0.3243554543208642639e-2_f64 * t33341 - 0.61320337121513228211e-3_f64 * t33343 + 0.22466860691349365008e-6_f64 * t33346 + 0.11594181388521408695e-4_f64 * t33349;
    (t36535, t36536, t36537, t36538, t36539, t36540, t36542, t36543, t36556)
}
