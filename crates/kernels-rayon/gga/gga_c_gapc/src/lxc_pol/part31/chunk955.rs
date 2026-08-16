//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 955/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk955(t10000: f64, t10003: f64, t10006: f64, t10010: f64, t10014: f64, t10016: f64, t10019: f64, t10021: f64, t9986: f64, t9991: f64, t9993: f64, t9995: f64, t9997: f64) -> f64 {
    let t11020 = 0.29517957899305555558e-5_f64 * t9986 + 0.84410248952307505288e-7_f64 * t9991 - 0.2748593934505475288e-5_f64 * t9993 + 0.2162369695472428426e-1_f64 * t9995 + 0.56273499301538336858e-7_f64 * t9997 + 0.56273499301538336858e-7_f64 * t10000 - 0.27801896084645508334e-2_f64 * t10003 - 0.78385901460875530441e-2_f64 * t10006 + 0.10136107947527008247e-3_f64 * t10010 + 0.14648281543675415196e-4_f64 * t10014 + 0.15176747947735985782e-5_f64 * t10016 + 0.55603792169291016668e-2_f64 * t10019 - 0.13913017666225690434e-3_f64 * t10021;
    t11020
}
