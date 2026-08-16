//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1022/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1022(t2205: f64, t6849: f64, t311: f64, t103: f64, t314: f64, t7875: f64, t7158: f64, t7591: f64, t8134: f64, t7877: f64, t875: f64, t15512: f64, t966: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17890 = 1.0_f64 / t6849 / t2205;
    let t17891 = t311 * t17890;
    let t17899 = t7875 * t314 * t103;
    let t18018 = t7591 * t7158 * t8134;
    let t18105 = t875 * t7877;
    let t18107 = t15512 * t966 * t18105 * t103;
    (t17890, t17891, t17899, t18018, t18105, t18107)
}
