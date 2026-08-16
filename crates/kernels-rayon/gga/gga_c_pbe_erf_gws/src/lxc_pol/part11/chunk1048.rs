//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1048/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1048(t12054: f64, t8833: f64, t13565: f64, t6484: f64, t13334: f64, t6: f64, t5: f64, t13571: f64, t6203: f64, t37997: f64, t9016: f64, t3128: f64, t38870: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44672 = t12054 * t8833;
    let t44695 = t6484 * t13565;
    let t44710 = t6 * t13334;
    let t44741 = t5 * t13334;
    let t44763 = t6203 * t13571;
    let t44814 = t9016 * t37997;
    let t44889 = t3128 * t38870;
    (t44672, t44695, t44710, t44741, t44763, t44814, t44889)
}
