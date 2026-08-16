//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1027/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1027(t1393: f64, t1459: f64, t1849: f64, t24932: f64, t26166: f64, t26170: f64, t26178: f64, t26181: f64, t26183: f64, t26505: f64, t27879: f64, t27888: f64, t27903: f64, t4037: f64, t4073: f64, t4077: f64, t574: f64, t652: f64, t7266: f64, t7412: f64, t8107: f64) -> f64 {
    let t27905 = t1393 * t8107 - 2.0_f64 * t1459 * t24932 - 2.0_f64 * t1459 * t27888 + t1849 * t7412 - 2.0_f64 * t27879 * t652 + t27903 * t574 - 2.0_f64 * t4037 * t7266 - 2.0_f64 * t4073 * t7266 - 2.0_f64 * t4077 * t7266 + t26166 + t26170 - t26178 - t26181 - t26183 + t26505;
    t27905
}
