//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1256/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1256(t213: f64, t80893: f64, t12328: f64, t2003: f64, t12248: f64, t59: f64, t1336: f64, t240: f64, t2690: f64, t6943: f64, t22865: f64, t6604: f64) -> (f64, f64, f64, f64, f64) {
    let t80894 = t80893 * t213;
    let t80899 = t2003 * t12328;
    let t80900 = 595.0_f64 / 5184.0_f64 * t80899;
    let t80901 = t12248 * t59;
    let t80903 = t1336 * t80901 * t240;
    let t80914 = t1336 * t6943 * t2690;
    let t80939 = t22865 * t6604;
    (t80894, t80900, t80903, t80914, t80939)
}
