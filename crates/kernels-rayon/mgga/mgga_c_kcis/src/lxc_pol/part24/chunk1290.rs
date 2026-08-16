//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1290/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1290(t14402: f64, t93426: f64, t95915: f64, t1071: f64, t1709: f64, t4547: f64, t95830: f64, t100297: f64, t100301: f64, t100307: f64, t100312: f64, t100340: f64, t100972: f64, t93425: f64, t93471: f64, t95816: f64, t95817: f64) -> (f64, f64, f64) {
    let t101136 = t93426 * t95915 * t14402;
    let t101141 = t95830 * t1709 * t1071 * t4547;
    let t101146 = 0.51485339506172839507e-4_f64 * t93471 - 0.33163888888888888888e-2_f64 * t100297 - 0.16581944444444444444e-2_f64 * t100301 - 0.55273148148148148147e-3_f64 * t100307 - 0.16581944444444444444e-2_f64 * t100312 - 0.61836467013888888889e-4_f64 * t93425 * t100972 - 0.61836467013888888889e-4_f64 * t93425 * t101136 - 0.12367293402777777778e-3_f64 * t93425 * t101141 + 0.49745833333333333332e-2_f64 * t100340 - t95816 - 0.7369753086419753086e-3_f64 * t95817;
    (t101136, t101141, t101146)
}
