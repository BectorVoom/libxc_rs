//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1281/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1281(t125424: f64, t125459: f64, t125482: f64, t125508: f64, t24574: f64, t34285: f64, t118136: f64, t118173: f64, t1186: f64, t24745: f64, t24849: f64, t27453: f64, t27477: f64, t27525: f64, t32457: f64, t32475: f64, t34295: f64, t34300: f64, t34301: f64, t3604: f64, t3624: f64, t470: f64, t4733: f64, t493: f64, t5064: f64, t5079: f64, t7283: f64, t7362: f64, t7373: f64, t7375: f64, t7376: f64) -> (f64, f64) {
    let t125510 = t125424 + t125459 + t125482 + t125508;
    let t125523 = t24574 * t34285;
    let t125530 = t3604 * t34301 + 0.16449340668482264365e-1_f64 * t7373 * t7375 * t27477 * t7376 - 0.16449340668482264365e-1_f64 * t7283 * t1186 * t34295 + t470 * t493 * t125510 - 0.18277045187202515961e-2_f64 * t118173 - 0.16449340668482264365e-1_f64 * t7283 * t27453 * t24745 * t32457 - t3624 * t34300 * t5079 - 0.54831135561607547883e-2_f64 * t24849 * t118136 * t27525 - 0.18277045187202515961e-2_f64 * t125523 - 0.54831135561607547883e-2_f64 * t7283 * t7362 * t32457 * t4733 + t5064 * t32475;
    (t125510, t125530)
}
