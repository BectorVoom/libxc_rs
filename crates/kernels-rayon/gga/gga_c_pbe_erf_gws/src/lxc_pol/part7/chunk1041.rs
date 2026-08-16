//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1041/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1041(t18753: f64, t18801: f64, t18839: f64, t18910: f64, t40: f64, t60: f64, t1336: f64, t1425: f64, t18639: f64, t18865: f64, t470: f64, t4737: f64) -> (f64, f64, f64) {
    let t18914 = t40 * t60 * (t18753 + t18801 + t18839 + t18910);
    let t18915 = t1336 * t1425;
    let t18916 = 144.0_f64 * t18915;
    let t18920 = 0.12304676425209353917e5_f64 * t470 * t18865 * t18639 * t4737;
    (t18914, t18916, t18920)
}
