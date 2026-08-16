//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1495/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1495(t45405: f64, t45545: f64, t112: f64, t12512: f64, t111: f64, t3931: f64, t12521: f64, t12524: f64, t12529: f64, t12532: f64, t1395: f64, t1401: f64, t16535: f64, t2319: f64, t2363: f64, t39231: f64, t3938: f64, t3941: f64, t45510: f64, t577: f64, t671: f64, t9416: f64) -> (f64, f64) {
    let t45546 = t45405 + t45545;
    let t45557 = t12512 * t112;
    let t45560 = t3931 * t111;
    let t45580 = 0.45e1_f64 * t45546 * t577 + 54.0_f64 * t45557 * t671 + 162.0_f64 * t45560 * t2319 + 81.0_f64 * t12521 * t2363 + 108.0_f64 * t1395 * t12529 + 324.0_f64 * t12524 * t12532 + 54.0_f64 * t3938 * t9416 + 162.0_f64 * t16535 * t2363 + 81.0_f64 * t3941 * t39231 + 108.0_f64 * t3941 * t671 * t9416 + 0.135e2_f64 * t1401 * t45510;
    (t45546, t45580)
}
