//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1105/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1105(t6048: f64, t7940: f64, t17308: f64, t2253: f64, t17311: f64, t7943: f64, t5897: f64, t7962: f64, t12338: f64, t8186: f64, t1555: f64, t12345: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28563 = t7940 * t6048;
    let t28564 = t17308 * t2253;
    let t28566 = 2.0_f64 * t17311 * t7943;
    let t28567 = t5897 * t7962;
    let t28569 = 2.0_f64 * t12338 * t8186;
    let t28570 = t8186 * t1555;
    let t28572 = 6.0_f64 * t12345 * t28570;
    (t28563, t28564, t28566, t28567, t28569, t28570, t28572)
}
