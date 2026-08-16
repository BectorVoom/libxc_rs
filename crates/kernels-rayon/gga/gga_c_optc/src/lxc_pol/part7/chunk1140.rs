//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1140/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1140(t2296: f64, t2301: f64, t2302: f64, t2315: f64, t23578: f64, t23618: f64, t23649: f64, t23687: f64, t23691: f64, t23694: f64, t23699: f64, t23708: f64, t23709: f64, t23715: f64, t23732: f64, t23745: f64, t23758: f64, t23771: f64, t350: f64, t8335: f64, t8338: f64, t8345: f64, t8346: f64, t8349: f64, t8376: f64, t974: f64, t979: f64) -> f64 {
    let t23775 = (t23578 + t23618 + t23649 + t23687) * t350 - 4.0_f64 * t23691 * t979 + 12.0_f64 * t23694 * t2302 - 6.0_f64 * t8335 * t2315 - 24.0_f64 * t23699 * t8346 + 24.0_f64 * t8338 * t8349 - 4.0_f64 * t2296 * t8376 + 24.0_f64 * t23708 * t23709 - 36.0_f64 * t8345 * t2302 * t2315 + 6.0_f64 * t2301 * t23715 + 8.0_f64 * t2301 * t979 * t8376 - t974 * (t23732 + t23745 + t23758 + t23771);
    t23775
}
