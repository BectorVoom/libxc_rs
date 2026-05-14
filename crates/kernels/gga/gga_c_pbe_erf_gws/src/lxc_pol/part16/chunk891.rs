//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 891/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk891<F: Float>(t2409: F, t831: F, t8759: F, t1161: F, t2074: F, t2376: F, t3062: F, t4414: F, t1115: F, t2379: F, t2408: F, t2498: F, t3040: F, t3066: F, t3207: F, t4487: F, t4489: F, t4496: F, t6145: F, t6784: F, t827: F, t8723: F, t8736: F, t8740: F, t8745: F, t8747: F, t8751: F, t8756: F) -> (F, F, F, F) {
    let t8761 = t2409 * t831 * t8759;
    let t8764 = t1161 * t2074;
    let t8766 = t2409 * t2376 * t8764;
    let t8771 = 7.0 / 72.0 * t4414 * t3062;
    let t8772 = t2408 * t8723 / 24.0 + t1115 * t6145 / 48.0 - t3040 * t2379 / 48.0 - t2498 * t2379 / 48.0 - t1115 * t6784 / 48.0 + t3066 * t8736 / 24.0 - t8740 + 35.0 / 216.0 * t4487 + 7.0 / 144.0 * t4489 - t8745 + 35.0 / 432.0 * t8747 + t827 * t8751 / 48.0 + t2408 * t8756 / 24.0 + t3207 * t8761 / 16.0 + t2408 * t8766 / 48.0 + 7.0 / 144.0 * t4496 - t8771;
    (t8761, t8764, t8766, t8772)
}
