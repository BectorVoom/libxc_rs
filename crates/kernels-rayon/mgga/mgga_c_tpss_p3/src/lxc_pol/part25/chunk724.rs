//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 724/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk724(t45: f64, t4716: f64, t773: f64, t774: f64, t1364: f64, t226: f64, t3629: f64, t2175: f64, t3643: f64, t2225: f64, t4573: f64, t4579: f64, t78: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t151 = t45 <= zeta_threshold;
    let t4718 = t773 * t774 * t4716;
    let t4722 = t226 * t1364;
    let t4723 = t3629 * t4722;
    let t4724 = t2175 * t4723;
    let t4727 = 8.0_f64 * t3643;
    let t4733 = piecewise3(t151, 0.0_f64, 4.0_f64 / 9.0_f64 * t2225 * t4573 + 4.0_f64 / 3.0_f64 * t78 * t4579);
    (t4718, t4722, t4724, t4727, t4733)
}
