//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 615/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk615(t1034: f64, t1044: f64, t164: f64, t167: f64, t183: f64, t2594: f64, t2639: f64, t2647: f64, t2670: f64, t2682: f64, t2693: f64, t588: f64, t600: f64, t621: f64) -> f64 {
    let t2702 = 0.13170898365871023197e1_f64 * t2682 * t2594 - 0.65854491829355115987e0_f64 * t588 * t621 * t1034 * t164 - 0.65854491829355115987e0_f64 * t588 * t183 * t2639 * t164 - 0.65854491829355115987e0_f64 * t2693 * t2647 - 0.65854491829355115987e0_f64 * t588 * t1044 * t600 * t164 + 0.65854491829355115987e0_f64 * t167 * t2670;
    t2702
}
