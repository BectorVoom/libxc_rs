//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 792/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk792(t366: f64, t6553: f64, t899: f64, t6242: f64, t904: f64, t916: f64, t2209: f64, t825: f64, t2182: f64, t337: f64, t5: f64, t2146: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6555 = t899 * t6553 * t366;
    let t6557 = t916 * t904 * t6242;
    let t6560 = t825 * t2209;
    let t6562 = t337 * t5 * t2182;
    let t6563 = t6560 * t6562;
    let t6565 = 3.0_f64 / 16.0_f64 * t2146 * t6563;
    (t6555, t6557, t6560, t6562, t6563, t6565)
}
