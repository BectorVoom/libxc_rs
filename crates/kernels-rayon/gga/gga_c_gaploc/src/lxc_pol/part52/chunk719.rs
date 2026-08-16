//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 719/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk719(t12574: f64, t13088: f64, t13517: f64, t13520: f64, t13521: f64, t13522: f64, t13523: f64, t13524: f64) -> f64 {
    let t14364 = t13517 + 2.0_f64 * t13088 - 2.0_f64 * t12574 - t13520 - t13521 + t13522 + t13523 + t13524;
    t14364
}
