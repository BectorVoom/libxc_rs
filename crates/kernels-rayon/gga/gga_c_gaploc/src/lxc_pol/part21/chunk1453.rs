//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1453/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1453(t12255: f64, t12318: f64, t1716: f64, t1897: f64, t1901: f64, t2042: f64, t2508: f64, t29354: f64, t32355: f64, t32360: f64, t32363: f64, t32370: f64, t32394: f64, t32398: f64, t32400: f64, t32408: f64, t32411: f64, t3722: f64, t39107: f64, t5227: f64) -> f64 {
    let t39420 = 0.15381052460284448567e-1_f64 * t1897 * t1901 * t39107 - t32355 - t32360 + t29354 + 0.17090058289204942853e-2_f64 * t5227 * t12318 - t32363 + 0.76905262301422242837e-2_f64 * t2508 * t2042 * t3722 - 0.23071578690426672851e-1_f64 * t2508 * t12255 * t1716 + t32370 - t32394 + t32398 - t32400 - t32408 + t32411;
    t39420
}
