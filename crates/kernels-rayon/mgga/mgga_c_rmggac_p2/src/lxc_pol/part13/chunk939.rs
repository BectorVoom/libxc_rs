//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 939/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk939(t2329: f64, t36669: f64, t1970: f64, t1971: f64, t209: f64, t40444: f64, t511: f64, t36662: f64, t8417: f64, t1986: f64, t305: f64, t495: f64, t552: f64) -> (f64, f64, f64, f64) {
    let t40647 = t36669 * t2329;
    let t40652 = t1970 * t1971 * t511 * t40444 * t209;
    let t40654 = t36662 * t8417;
    let t40658 = t1986 * t305 * t552 * t495;
    (t40647, t40652, t40654, t40658)
}
