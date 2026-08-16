//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1022/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1022(t40234: f64, t40237: f64, t40245: f64, t41968: f64, t41972: f64, t41975: f64, t41976: f64, t41978: f64, t41979: f64, t41980: f64, t41981: f64, t41982: f64, t41983: f64, t41984: f64, t41987: f64, t47987: f64, t47989: f64, t47995: f64, t47997: f64, t48001: f64) -> f64 {
    let t50880 = -0.14300195980740170668e1_f64 * t47987 - 0.92023022289409799224e1_f64 * t47989 + t41968 - t41972 - 0.89376224879626066675e-1_f64 * t40234 + 0.59584149919750711115e-1_f64 * t40237 - t41975 - t41976 - 0.76685851907841499353e0_f64 * t40245 + t41978 + t41979 - t41980 - t41981 + 0.47667319935800568892e0_f64 * t47995 - 0.13803453343411469884e2_f64 * t47997 - 0.13803453343411469884e2_f64 * t48001 + t41982 - t41983 + t41984 - t41987;
    t50880
}
