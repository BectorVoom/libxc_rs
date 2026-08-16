//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1353/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1353(t1555: f64, t7271: f64, t12345: f64, t2069: f64, t6048: f64, t4189: f64, t4184: f64, t7397: f64, t1529: f64, t7386: f64, t1543: f64, t7329: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22310 = t7271 * t1555;
    let t22312 = 6.0_f64 * t12345 * t22310;
    let t22313 = t2069 * t6048;
    let t22315 = 4.0_f64 * t4189 * t22313;
    let t22316 = t4184 * t7397;
    let t22317 = t7397 * t1555;
    let t22319 = 2.0_f64 * t4189 * t22317;
    let t22320 = t1529 * t7386;
    let t22322 = t1543 * t7329;
    (t22312, t22315, t22316, t22319, t22320, t22322)
}
