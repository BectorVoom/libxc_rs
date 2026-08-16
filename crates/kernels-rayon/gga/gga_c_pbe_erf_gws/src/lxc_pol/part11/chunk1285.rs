//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1285/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1285(t49943: f64, t49945: f64, t49950: f64, t49952: f64, t49954: f64, t49963: f64, t49980: f64, t49986: f64, t50027: f64, t50036: f64, t50041: f64, t50043: f64, t50045: f64, t50049: f64, t50051: f64, t50056: f64, t50073: f64, t50077: f64, t50087: f64, t50103: f64, t50107: f64, t50109: f64) -> (f64, f64) {
    let t50582 = -t49943 - t49945 + t49950 + t49952 + t49954 + t49963 + t49980 + t49986 - t50027 + t50036 + t50041;
    let t50583 = -t50043 + t50045 - t50049 + t50051 + t50056 - t50073 + t50077 + t50087 - t50103 - t50107 - t50109;
    (t50582, t50583)
}
