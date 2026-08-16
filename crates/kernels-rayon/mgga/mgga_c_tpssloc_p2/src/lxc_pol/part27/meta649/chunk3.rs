//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2253/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2253(t25: f64, t265: f64, t394: f64, t89775: f64, t89822: f64, t89729: f64, t12606: f64, t1409: f64, t1965: f64, t2250: f64, t23773: f64, t25883: f64, t3966: f64, t40: f64, t607: f64, t6835: f64, t7643: f64, t88003: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t89823 = t89775 + t89822;
    let t89824 = piecewise3(t395, t89729, t89823);
    let t89836 = piecewise3(t115, t88003, t89824 * t40 / 2.0_f64 + t25883 * t607 + t7643 * t2250 / 2.0_f64 + t23773 * t1409 / 2.0_f64 + t6835 * t3966 + t1965 * t12606 / 2.0_f64);
    (t89823, t89836)
}
