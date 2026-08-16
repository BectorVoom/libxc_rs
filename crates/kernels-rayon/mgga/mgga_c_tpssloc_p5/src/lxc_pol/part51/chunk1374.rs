//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1374/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1374(t191: f64, t192: f64, t27215: f64, t2020: f64, t26142: f64, t7042: f64, t25010: f64, t8607: f64, t23938: f64, t7468: f64, t120067: f64, t19456: f64, t2040: f64, t26878: f64, t27150: f64, t31057: f64, t31060: f64, t31726: f64, t4028: f64, t6517: f64, t652: f64, t7056: f64, t7670: f64, t8450: f64, t8529: f64, t90400: f64) -> f64 {
    let t121210 = t27215 * t191 * t192;
    let t121211 = t121210 * t2020;
    let t121224 = 2.0_f64 * t7042 * t26142;
    let t121226 = t8607 * t25010;
    let t121228 = 2.0_f64 * t23938 * t7468;
    let t121229 = -2.0_f64 * t652 * t7056 * t7670 - 2.0_f64 * t19456 * t8529 - 2.0_f64 * t2040 * t90400 - t26878 * t8450 - 2.0_f64 * t27150 * t6517 - 2.0_f64 * t31726 * t4028 - t120067 + t121211 - t121224 - t121226 - t121228 - t31057 - t31060;
    t121229
}
