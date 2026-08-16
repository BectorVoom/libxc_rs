//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 962/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk962(t2253: f64, t5442: f64, t10838: f64, t10921: f64, t14421: f64, t14423: f64, t14429: f64, t14431: f64, t14445: f64, t14448: f64, t14478: f64, t14480: f64, t14482: f64, t18820: f64, t18823: f64, t18825: f64, t18854: f64, t18859: f64, t18864: f64, t18867: f64, t18871: f64, t2265: f64, t631: f64) -> f64 {
    let t18874 = t2253 * t5442;
    let t18876 = t14421 + t14423 + 4.0_f64 / 9.0_f64 * t14429 + 10.0_f64 / 27.0_f64 * t14431 + 10.0_f64 / 9.0_f64 * t14445 - t14448 + t14478 + t14480 - t14482 + t10838 + 5.0_f64 / 27.0_f64 * t10921 - 2.0_f64 / 3.0_f64 * t2265 * t18820 - t18823 / 3.0_f64 + t18825 + t631 * t18854 / 2.0_f64 - 2.0_f64 / 9.0_f64 * t2265 * t18859 + 2.0_f64 * t2265 * t18864 + 4.0_f64 / 3.0_f64 * t2265 * t18867 - 3.0_f64 * t631 * t18871 - t18874 / 27.0_f64;
    t18876
}
