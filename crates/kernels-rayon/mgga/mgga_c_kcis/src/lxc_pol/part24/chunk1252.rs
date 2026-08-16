//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1252/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1252(t1092: f64, t1121: f64, t6689: f64, t95655: f64, t1133: f64, t27763: f64, t70032: f64, t1020: f64, t19706: f64, t7718: f64, t100436: f64, t26955: f64, t26966: f64, t27070: f64, t28184: f64, t28204: f64, t29112: f64, t29161: f64, t8087: f64, t96977: f64, t96980: f64, t96993: f64, t97010: f64) -> (f64, f64, f64, f64) {
    let t100447 = t1092 * t95655 * t6689 * t1121;
    let t100451 = t1092 * t27763 * t70032 * t1133;
    let t100456 = t1020 * t7718 * t19706;
    let t100458 = -0.30918233506944444445e-4_f64 * t26955 * t100436 + t96977 - t96980 - 0.92754700520833333334e-4_f64 * t27070 * t29161 - 0.92754700520833333334e-4_f64 * t28204 * t28184 + t96993 - 0.18534722222222222222e-2_f64 * t97010 * t8087 - 0.10446625e-1_f64 * t100447 + 0.23214722222222222221e-2_f64 * t100451 + 0.41188271604938271605e-3_f64 * t26966 * t29112 + 0.77382407407407407407e-3_f64 * t100456;
    (t100447, t100451, t100456, t100458)
}
