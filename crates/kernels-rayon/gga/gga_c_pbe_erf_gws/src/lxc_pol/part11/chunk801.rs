//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 801/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk801(t12898: f64, t506: f64, t102: f64, t10: f64, t127: f64, t12931: f64, t12934: f64, t12937: f64, t12946: f64, t12947: f64, t12951: f64, t12952: f64, t12955: f64, t2893: f64, t3637: f64, t496: f64, t5836: f64, t8149: f64, t8160: f64, t8200: f64) -> (f64, f64, f64) {
    let t12958 = t506 * t12898;
    let t12960 = 0.1753815e2_f64 * t102 * t12958;
    let t12961 = -t496 * t12931 / 2.0_f64 + t12934 - 0.293808e1_f64 * t8149 - 0.146904e1_f64 * t8160 + 9.0_f64 / 2.0_f64 * t496 * t10 * t12937 + 0.1762848e2_f64 * t127 * t2893 * t3637 + t12946 - t12947 - 2.0_f64 / 3.0_f64 * t8200 + t5836 - t12951 - 0.146904e1_f64 * t127 * t12952 - 0.293808e2_f64 * t127 * t12955 - t12960;
    (t12958, t12960, t12961)
}
