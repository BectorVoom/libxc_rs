//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1848/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1848(t26384: f64, t6637: f64, t6888: f64, t5187: f64, t6968: f64, t22893: f64, t7732: f64, t22892: f64, t1834: f64, t552: f64, t1307: f64, t26328: f64, t553: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26385 = t6637 * t26384;
    let t26386 = t6888 * t26385;
    let t26388 = t6968 * t5187;
    let t26389 = t6637 * t26388;
    let t26390 = t6888 * t26389;
    let t26392 = t22893 * t7732;
    let t26393 = t22892 * t26392;
    let t26395 = t552 * t1834;
    let t26396 = t26395 * t1307;
    let t26397 = t6637 * t26396;
    let t26398 = t6888 * t26397;
    let t26401 = t553 * t26328;
    (t26385, t26386, t26388, t26389, t26390, t26392, t26393, t26395, t26396, t26397, t26398, t26401)
}
