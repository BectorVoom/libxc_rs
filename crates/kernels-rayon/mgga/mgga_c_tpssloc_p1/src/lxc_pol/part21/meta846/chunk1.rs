//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3062/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3062(t11424: f64, t18255: f64, t1117: f64, t18835: f64, t3264: f64, t3307: f64, t6021: f64, t11190: f64, t18258: f64, t3265: f64, t11185: f64, t18259: f64) -> (f64, f64, f64, f64, f64) {
    let t63576 = 4.0_f64 * t11424 * t18255;
    let t63579 = 4.0_f64 * t3264 * t18835 * t1117;
    let t63582 = 2.0_f64 * t3264 * t6021 * t3307;
    let t63585 = 0.96491876992155210402e2_f64 * t11190 * t18258 * t3265;
    let t63587 = 0.32163958997385070134e2_f64 * t11185 * t18259;
    (t63576, t63579, t63582, t63585, t63587)
}
