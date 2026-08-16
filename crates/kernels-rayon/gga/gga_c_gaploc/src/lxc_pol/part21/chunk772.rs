//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 772/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk772(t2580: f64, t7305: f64, t2549: f64, t2564: f64, t5508: f64, t883: f64, t732: f64, t1877: f64, t481: f64, t941: f64, t2042: f64, t2558: f64) -> (f64, f64, f64, f64, f64) {
    let t7306 = t2580 * t7305;
    let t7309 = t2549 * t2564;
    let t7313 = t883 * t5508;
    let t7314 = t732 * t7313;
    let t7315 = t481 * t941 * t1877 * t7314;
    let t7317 = t2042 * t2558;
    (t7306, t7309, t7313, t7315, t7317)
}
