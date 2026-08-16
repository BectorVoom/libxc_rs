//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1344/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1344(t2016: f64, t4188: f64, t4190: f64, t4310: f64, t5897: f64, t12335: f64, t2069: f64, t12338: f64, t5900: f64, t4184: f64, t6048: f64, t12345: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17311 = t2016 * t4188;
    let t17313 = 2.0_f64 * t17311 * t4190;
    let t17314 = t5897 * t4310;
    let t17315 = t12335 * t2069;
    let t17317 = 4.0_f64 * t12338 * t5900;
    let t17319 = 2.0_f64 * t4184 * t6048;
    let t17320 = t2069 * t4190;
    let t17322 = 6.0_f64 * t12345 * t17320;
    (t17313, t17314, t17315, t17317, t17319, t17322)
}
