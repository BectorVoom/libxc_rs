//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 733/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk733(t12: f64, t1646: f64, t5093: f64, t5094: f64, t5097: f64, t5100: f64, t87: f64, t139: f64, t24: f64, t1652: f64, t507: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t84 = t12 <= zeta_threshold;
    let t5104 = piecewise3(t84, 0.0_f64, -8.0_f64 / 27.0_f64 * t5093 * t5094 + 4.0_f64 / 3.0_f64 * t5097 * t1646 + 4.0_f64 / 3.0_f64 * t87 * t5100);
    let t5106 = 1.0_f64 / t139 / t24;
    let t5107 = t1652 * t507;
    (t5104, t5106, t5107)
}
