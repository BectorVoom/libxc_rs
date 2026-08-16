//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 857/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk857(t17338: f64, t4822: f64, t558: f64, t12791: f64, t17334: f64, t11755: f64, t11761: f64, t12852: f64, t12864: f64, t12865: f64, t17296: f64, t17299: f64, t17302: f64, t17305: f64, t17310: f64, t17313: f64, t17316: f64, t17319: f64, t17322: f64, t17325: f64, t17328: f64, t17331: f64, t17335: f64, t3139: f64, t462: f64, t9178: f64, t9202: f64) -> f64 {
    let t17340 = t17338 * t4822 * t558;
    let t17343 = t12791 * t17334;
    let t17346 = -10.0_f64 / 27.0_f64 * t462 * t17296 - 8.0_f64 / 9.0_f64 * t3139 * t17299 + 2.0_f64 / 3.0_f64 * t462 * t17302 + t462 * t17305 / 3.0_f64 - 8.0_f64 / 27.0_f64 * t12852 - t12864 + 4.0_f64 / 9.0_f64 * t12865 - t9178 - 2.0_f64 / 9.0_f64 * t17310 - 4.0_f64 / 27.0_f64 * t9202 - 2.0_f64 / 3.0_f64 * t462 * t17313 + t462 * t17316 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t462 * t17319 - 2.0_f64 / 9.0_f64 * t462 * t17322 - 2.0_f64 / 3.0_f64 * t462 * t17325 - 2.0_f64 * t462 * t17328 + 8.0_f64 / 3.0_f64 * t3139 * t17331 + 4.0_f64 / 9.0_f64 * t11755 * t17335 - 4.0_f64 / 3.0_f64 * t11761 * t17340 - 4.0_f64 / 3.0_f64 * t11761 * t17343;
    t17346
}
