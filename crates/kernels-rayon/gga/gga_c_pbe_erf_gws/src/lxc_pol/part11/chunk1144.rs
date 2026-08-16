//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1144/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1144(t2559: f64, t47442: f64, t587: f64, t12613: f64, t7527: f64, t1620: f64, t1809: f64, t32114: f64, t3351: f64, t48169: f64, t48173: f64, t48175: f64, t48179: f64, t48183: f64, t48187: f64, t48191: f64, t48195: f64, t48198: f64) -> (f64, f64, f64, f64) {
    let t48201 = 16.0_f64 / 3.0_f64 * t587 * t2559 * t47442;
    let t48203 = 32.0_f64 / 15.0_f64 * t7527 * t12613;
    let t48207 = 32.0_f64 / 15.0_f64 * t1620 * t1809 * t32114 * t3351;
    let t48208 = t48169 + t48173 - t48175 - t48179 + t48183 + t48187 - t48191 - t48195 - t48198 - t48201 + t48203 - t48207;
    (t48201, t48203, t48207, t48208)
}
