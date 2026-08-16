//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 786/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk786(t36541: f64, t674: f64, t7269: f64, t7508: f64, t2084: f64, t2145: f64, t27: f64, t866: f64, t1347: f64, t2153: f64, t1987: f64, t7939: f64) -> (f64, f64, f64, f64, f64) {
    let t36542 = t36541 * t674;
    let t36590 = t7508 * t7269;
    let t36594 = t2145 * t27 * t2084 * t866;
    let t36601 = t1347 * t2153;
    let t36610 = t7939 * t1987;
    (t36542, t36590, t36594, t36601, t36610)
}
