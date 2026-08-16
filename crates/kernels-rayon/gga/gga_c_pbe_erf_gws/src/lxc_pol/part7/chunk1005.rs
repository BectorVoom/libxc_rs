//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1005/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1005(t17525: f64, t17531: f64, t17533: f64, t17536: f64, t17539: f64, t17543: f64, t17545: f64, t17559: f64, t17562: f64, t17565: f64, t17567: f64, t1914: f64, t5434: f64) -> (f64, f64) {
    let t18304 = -t17525 + t17531 + t17533 - t17536 - t17539 + t17543 + t17545 + t17559 + t17562 - t17565 + t17567;
    let t18305 = t1914 * t5434;
    (t18304, t18305)
}
