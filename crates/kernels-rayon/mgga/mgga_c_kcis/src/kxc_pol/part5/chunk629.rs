//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 629/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk629(t1444: f64, t538: f64, t1466: f64, t1527: f64, t4121: f64, t569: f64, t1532: f64, t492: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4230 = t538 * t1444;
    let t4248 = t1527 * t1466;
    let t4249 = t4248 * sigma2;
    let t4254 = t569 * t4121;
    let t4255 = t4254 * sigma2;
    let t4260 = t1532 * t492;
    (t4230, t4248, t4249, t4254, t4255, t4260)
}
