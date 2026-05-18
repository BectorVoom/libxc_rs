//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1103/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1103<F: Float>(t19733: F, t2382: F, t833: F, t4867: F, t823: F, t825: F, t2112: F, t328: F, t331: F, t745: F, t2416: F, t810: F) -> (F, F, F, F, F, F, F, F) {
    let t19735 = t2382 * t19733 * t833;
    let t19737 = t823 * t4867;
    let t19738 = t19737 * t825;
    let t19744 = t2112 * t328;
    let t19745 = t19744 * t331;
    let t19750 = t745 * t328;
    let t19751 = t19750 * t331;
    let t19756 = t2416 * t810;
    (t19735, t19737, t19738, t19744, t19745, t19750, t19751, t19756)
}
