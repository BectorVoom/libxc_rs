//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 901/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk901(t6241: f64, t6255: f64, t1629: f64, t1636: f64, t187: f64, t2128: f64, t4475: f64, t4480: f64, t5896: f64, t5898: f64, t5899: f64, t5902: f64, t6049: f64, t6220: f64, t6222: f64, t6225: f64, t633: f64) -> (f64, f64) {
    let t6256 = t6241 + t6255;
    let t6260 = t5896 - t5898 - t5899 + t5902 - t6049 + t187 * (-t1629 * t6256 - t1636 * t6222 - t2128 * t4475 + 2.0_f64 * t4480 * t6225 + t6220 * t633 - t5896 + t5898 + t5899 - t5902 + t6049);
    (t6256, t6260)
}
