//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 983/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk983(t2409: f64, t831: f64, t8759: f64, t1161: f64, t2074: f64, t2376: f64, t3062: f64, t4414: f64, t1115: f64, t2379: f64, t2408: f64, t2498: f64, t3040: f64, t3066: f64, t3207: f64, t4487: f64, t4489: f64, t4496: f64, t6145: f64, t6784: f64, t827: f64, t8723: f64, t8736: f64, t8740: f64, t8745: f64, t8747: f64, t8751: f64, t8756: f64) -> (f64, f64, f64, f64) {
    let t8761 = t2409 * t831 * t8759;
    let t8764 = t1161 * t2074;
    let t8766 = t2409 * t2376 * t8764;
    let t8771 = 7.0_f64 / 72.0_f64 * t4414 * t3062;
    let t8772 = t2408 * t8723 / 24.0_f64 + t1115 * t6145 / 48.0_f64 - t3040 * t2379 / 48.0_f64 - t2498 * t2379 / 48.0_f64 - t1115 * t6784 / 48.0_f64 + t3066 * t8736 / 24.0_f64 - t8740 + 35.0_f64 / 216.0_f64 * t4487 + 7.0_f64 / 144.0_f64 * t4489 - t8745 + 35.0_f64 / 432.0_f64 * t8747 + t827 * t8751 / 48.0_f64 + t2408 * t8756 / 24.0_f64 + t3207 * t8761 / 16.0_f64 + t2408 * t8766 / 48.0_f64 + 7.0_f64 / 144.0_f64 * t4496 - t8771;
    (t8761, t8764, t8766, t8772)
}
