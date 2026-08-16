//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 761/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk761(t15860: f64, t419: f64, t1725: f64, t4488: f64, t173: f64, t4487: f64, t15763: f64, t3088: f64, t1527: f64, t15768: f64, t15625: f64, t423: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15861 = t419 * t15860;
    let t15863 = t1725 * t4488;
    let t15865 = t173 * t4487;
    let t15866 = t419 * t15865;
    let t15868 = t3088 * t15763;
    let t15869 = t419 * t15868;
    let t15871 = t1527 * t15768;
    let t15872 = t419 * t15871;
    let t15874 = t423 * t15625;
    (t15861, t15863, t15866, t15869, t15872, t15874)
}
