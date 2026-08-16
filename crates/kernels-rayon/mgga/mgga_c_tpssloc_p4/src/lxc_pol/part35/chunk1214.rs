//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1214/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1214(t3: f64, t5398: f64, t1915: f64, t5527: f64, t1484: f64, t1530: f64, t1877: f64, t193: f64, t202: f64, t23295: f64, t2522: f64, t25358: f64, t28248: f64, t28447: f64, t4314: f64, t5544: f64, t5660: f64, t5664: f64, t6670: f64, t7541: f64, t870: f64) -> (f64, f64) {
    let t28525 = t3 * t5398;
    let t28732 = t1915 * t5527;
    let t28755 = t193 * t202 * t28447 * t870 + 6.0_f64 * t1484 * t2522 * t7541 - 2.0_f64 * t1530 * t1877 * t25358 + 2.0_f64 * t1877 * t23295 * t5664 - t1877 * t5660 * t6670 + 3.0_f64 * t1915 * t2522 * t5544 - 6.0_f64 * t2522 * t28248 * t6670 + 6.0_f64 * t28732 * t4314;
    (t28525, t28755)
}
