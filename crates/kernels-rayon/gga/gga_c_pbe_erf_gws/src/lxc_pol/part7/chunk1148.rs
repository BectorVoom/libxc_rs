//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1148/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1148(t6658: f64, t825: f64, t337: f64, t5: f64, t6385: f64, t2146: f64, t6319: f64, t6707: f64, t6257: f64, t6331: f64, t2112: f64, t4394: f64) -> (f64, f64, f64, f64) {
    let t20560 = t825 * t6658;
    let t20563 = t20560 * t337 * t5 * t6385;
    let t20564 = t2146 * t20563;
    let t20566 = t6319 * t6707 / 32.0_f64;
    let t20567 = t6331 * t6257;
    let t20568 = t2146 * t20567;
    let t20569 = 7.0_f64 / 12.0_f64 * t20568;
    let t20571 = t2112 * t4394;
    (t20564, t20566, t20569, t20571)
}
