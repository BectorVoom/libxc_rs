//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 947/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk947(t10000: f64, t14212: f64, t14223: f64, t14224: f64, t14232: f64, t14233: f64, t18602: f64, t18606: f64, t18610: f64, t18614: f64, t18619: f64, t18624: f64, t18628: f64, t18633: f64, t18636: f64, t446: f64) -> f64 {
    let t18639 = 4.0_f64 / 27.0_f64 * t10000 - t446 * t18602 / 9.0_f64 - t446 * t18606 / 9.0_f64 - 2.0_f64 / 27.0_f64 * t446 * t18610 - 2.0_f64 / 9.0_f64 * t446 * t18614 - 2.0_f64 / 3.0_f64 * t446 * t18619 + 4.0_f64 / 3.0_f64 * t446 * t18624 + 2.0_f64 / 3.0_f64 * t446 * t18628 + t14212 - t14223 - 8.0_f64 / 81.0_f64 * t14224 + t14232 - 8.0_f64 / 27.0_f64 * t14233 - 2.0_f64 / 9.0_f64 * t18633 - t446 * t18636 / 9.0_f64;
    t18639
}
