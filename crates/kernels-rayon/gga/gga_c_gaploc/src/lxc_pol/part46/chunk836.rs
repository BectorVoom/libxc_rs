//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 836/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk836(t1: f64, t41801: f64, t544: f64, t1424: f64, t2386: f64, t40116: f64, t41752: f64, t41753: f64, t41754: f64, t41759: f64, t41761: f64, t41763: f64, t41767: f64, t41769: f64, t41773: f64, t41777: f64, t41781: f64, t41783: f64, t41787: f64, t41790: f64, t41793: f64, t41794: f64, t41798: f64, t41800: f64) -> f64 {
    let t41803 = t544 * t41801 * t1;
    let t41806 = -t41752 - t41753 + t41754 - 0.85206502119823888169e-1_f64 * t40116 - t41759 + t41761 - 0.10725146985555128001e1_f64 * t41763 * t2386 + t41767 - 0.18404604457881959845e2_f64 * t41769 - t41773 + t41777 + t41781 - t41783 - t41787 - t41790 + t41793 - 0.92023022289409799224e1_f64 * t41794 - 0.92023022289409799224e1_f64 * t41798 + t41800 - 0.39722766613167140743e-1_f64 * t41803 * t1424;
    t41806
}
