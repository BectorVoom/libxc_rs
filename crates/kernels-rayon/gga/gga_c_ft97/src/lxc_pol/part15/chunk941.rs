//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 941/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk941(t1882: f64, t20184: f64, t20244: f64, t20236: f64, t8392: f64, t20417: f64, t20397: f64, t20395: f64, t487: f64, t20172: f64, t20265: f64, t20210: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t74863 = t1882 * t20184;
    let t74865 = t1882 * t20244;
    let t74883 = t8392 * t20236;
    let t74899 = t1882 * t20417;
    let t74902 = t1882 * t20397;
    let t74959 = t487 * t20395;
    let t74992 = t8392 * t20172;
    let t75034 = t1882 * t20265;
    let t75048 = t8392 * t20210;
    (t74863, t74865, t74883, t74899, t74902, t74959, t74992, t75034, t75048)
}
