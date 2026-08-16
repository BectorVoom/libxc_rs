//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1298/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1298(t13045: f64, t22238: f64, t787: f64, t10930: f64, t10931: f64, t32803: f64, t16687: f64, t19: f64, t60: f64, t822: f64, t16692: f64, t201: f64) -> (f64, f64, f64, f64) {
    let t33325 = 0.53625734927775640005e1_f64 * t787 * t22238 * t13045;
    let t33328 = 0.38649669361552115674e3_f64 * t10930 * t10931 * t32803;
    let t33331 = t822 * t16687 * t19 * t60;
    let t33332 = t201 * t16692;
    (t33325, t33328, t33331, t33332)
}
