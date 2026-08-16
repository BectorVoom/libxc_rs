//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 723/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk723(t1202: f64, t3721: f64, t333: f64, t3724: f64, t317: f64, t3675: f64, t305: f64, t3951: f64, t79: f64, t222: f64, t3531: f64) -> (f64, f64, f64, f64, f64) {
    let t12884 = 1.0_f64 / t3721 / t1202;
    let t12888 = 1.0_f64 / t3724 / t333;
    let t12909 = 1.0_f64 / t3675 / t317;
    let t12910 = t305 * t12909;
    let t12941 = t79 * t3951;
    let t12951 = 1.0_f64 / t3531 / t222;
    (t12884, t12888, t12910, t12941, t12951)
}
