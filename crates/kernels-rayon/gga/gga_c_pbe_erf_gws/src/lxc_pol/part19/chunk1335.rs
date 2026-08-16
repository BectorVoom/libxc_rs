//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1335/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1335(t15159: f64, t3111: f64, t833: f64, t850: f64, t13796: f64, t13798: f64, t3989: f64, t57451: f64, t12234: f64, t3952: f64, t2080: f64, t3803: f64, t51502: f64) -> (f64, f64, f64, f64) {
    let t57458 = t850 * t3111 * t15159 * t833;
    let t57462 = t3989 * t13796 * t57451 * t13798;
    let t57468 = t3952 * t12234;
    let t57472 = t2080 * t3803 * t51502 * t833;
    (t57458, t57462, t57468, t57472)
}
