//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 750/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk750(t1504: f64, t2880: f64, t8589: f64, t8557: f64, t8560: f64, t8564: f64, t8568: f64, t8572: f64, t8575: f64, t8579: f64, t8581: f64, t8583: f64, t8586: f64) -> (f64, f64) {
    let t8590 = t2880 * t1504;
    let t8591 = t8589 * t8590;
    let t8593 = 0.43449121406768801912e-4_f64 * t8557 + 0.21724560703384400956e-4_f64 * t8560 + 0.12672660410307567224e-4_f64 * t8564 - 0.43449121406768801912e-4_f64 * t8568 + 0.12672660410307567224e-4_f64 * t8572 - 0.12360406057797588768e-3_f64 * t8575 - 0.43449121406768801912e-4_f64 * t8579 + 0.27517776890953574544e-3_f64 * t8581 + 0.86596512803768376033e-4_f64 * t8583 - 0.10427789137624512459e-2_f64 * t8586 + 0.20855578275249024918e-2_f64 * t8591;
    (t8591, t8593)
}
