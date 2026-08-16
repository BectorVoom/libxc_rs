//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1013/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1013(t148: f64, t18411: f64, t1464: f64, t700: f64, t242: f64, t5676: f64, t5984: f64, t5668: f64, t1286: f64, t174: f64, t4708: f64, t155: f64, t4658: f64, t4662: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t18413 = 0.83762820535504401876e-1_f64 * t148 * t18411;
    let t18415 = 0.2010307692852105645e1_f64 * t1464 * t700;
    let t18416 = t5676 * t242;
    let t18419 = 0.2010307692852105645e1_f64 * t5984 * t242;
    let t18420 = t5668 * t242;
    let t18424 = 0.14246666666666666667e0_f64 * t174 * t4708 * t1286;
    let t18428 = 0.36845452142031360636e2_f64 * t174 * t155 * t4658 * t4662;
    (t18413, t18415, t18416, t18419, t18420, t18424, t18428)
}
