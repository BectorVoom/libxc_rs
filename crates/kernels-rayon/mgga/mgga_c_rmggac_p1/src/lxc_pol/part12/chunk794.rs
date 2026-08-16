//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 794/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk794(t7244: f64, t7463: f64, t1255: f64, t1986: f64, t1034: f64, t132: f64, t7933: f64, t7934: f64, t303: f64, t388: f64, t357: f64, t7334: f64, t7932: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t36893 = t7244 * t7463;
    let t36895 = t1986 * t1255;
    let t36902 = t7933 * t7934 * t1034 * t132;
    let t36906 = t7933 * t7934 * t388 * t303;
    let t36910 = t7933 * t7934 * t388 * t357;
    let t36912 = t7334 * t7932;
    (t36893, t36895, t36902, t36906, t36910, t36912)
}
