//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1400/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1400(t12217: f64, t617: f64, t16055: f64, t16905: f64, t16065: f64, t1928: f64, t610: f64, t990: f64, t4455: f64, t6183: f64, t1610: f64, t6176: f64) -> (f64, f64, f64, f64) {
    let t18183 = t12217 * t617;
    let t18184 = t18183 * t16055;
    let t18187 = t16905 * t617;
    let t18188 = t18187 * t16065;
    let t18192 = t610 * t1928 * t990;
    let t18195 = t4455 * t6183;
    let t18196 = t18195 * t1610;
    let t18197 = t6176 * t18196;
    (t18184, t18188, t18192, t18197)
}
