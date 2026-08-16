//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 633/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk633(t1899: f64, t3554: f64, t1905: f64, t2730: f64, t3517: f64, t1095: f64, t703: f64) -> (f64, f64, f64, f64) {
    let t3556 = 0.16081979498692535067e2_f64 * t1899 * t3554;
    let t3559 = t1905 - 0.34246666666666666666e-1_f64 * t2730 + 0.5137e-1_f64 * t3517;
    let t3564 = t1095 * t1095;
    let t3565 = t3564 * t703;
    (t3556, t3559, t3564, t3565)
}
