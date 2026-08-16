//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 956/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk956(t2191: f64, t9938: f64, t10040: f64, t2004: f64, t9826: f64, t2007: f64, t1987: f64, t7501: f64, t9799: f64, t2139: f64, t27: f64, t6376: f64, t649: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t45864 = t2191 * t9938;
    let t45866 = t2191 * t10040;
    let t45869 = t9826 * t2004;
    let t45872 = t9826 * t2007;
    let t45874 = t9826 * t1987;
    let t45880 = t7501 * t9799;
    let t45884 = t2139 * t27 * t649 * t6376;
    (t45864, t45866, t45869, t45872, t45874, t45880, t45884)
}
