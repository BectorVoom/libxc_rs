//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1127/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1127(t16399: f64, t8916: f64, t164: f64, t8888: f64, t5257: f64, t8906: f64, t6966: f64, t8911: f64, t17053: f64, t3418: f64, t8897: f64, t1769: f64, t8823: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24298 = t16399 * t8916;
    let t24300 = t8888 * t164;
    let t24320 = t5257 * t8906;
    let t24322 = t6966 * t8911;
    let t24347 = t17053 * t3418;
    let t24370 = t5257 * t8897;
    let t24381 = t1769 * t8823;
    (t24298, t24300, t24320, t24322, t24347, t24370, t24381)
}
