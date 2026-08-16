//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 618/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk618(t26: f64, t8570: f64, t4711: f64, t4723: f64, t6756: f64, t6823: f64, t8512: f64, t8516: f64, t8520: f64, t8525: f64, t8527: f64, t8559: f64, t8561: f64, t8565: f64, t8568: f64) -> (f64, f64) {
    let t8571 = t26 * t8570;
    let t8573 = -0.9494625e0_f64 * t8525 + 0.1898925e1_f64 * t8527 + t4711 + 0.19931111111111111111e0_f64 * t6756 - 0.19931111111111111111e0_f64 * t8512 + 0.59793333333333333334e0_f64 * t8516 - 0.29896666666666666667e0_f64 * t8520 + 0.15358125e0_f64 * t8559 + 0.3071625e0_f64 * t8561 + t4723 + 0.10954222222222222222e0_f64 * t6823 - 0.27385555555555555556e-1_f64 * t8565 + 0.16431333333333333333e0_f64 * t8568 - 0.82156666666666666667e-1_f64 * t8571;
    (t8571, t8573)
}
