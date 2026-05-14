//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 984/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk984<F: Float>(t19737: F, t825: F, t2112: F, t328: F, t331: F, t745: F, t2416: F, t810: F, t2417: F, t353: F, t4386: F, t19615: F, t814: F, t859: F, t19608: F, t19714: F, t19722: F, t19726: F, t19728: F, t19731: F, t19735: F, t2118: F, t2362: F, t2382: F, t2388: F, t2397: F, t3074: F, t3079: F, t4395: F, t6112: F, t6135: F, t6158: F, t6793: F, t6802: F, t822: F, t833: F) -> (F, F, F, F, F, F) {
    let t19738 = t19737 * t825;
    let t19744 = t2112 * t328;
    let t19745 = t19744 * t331;
    let t19750 = t745 * t328;
    let t19751 = t19750 * t331;
    let t19756 = t2416 * t810;
    let t19759 = t4386 * t353 * t19756 * t2417;
    let t19764 = t859 * t353 * t19615 * t814;
    let t19767 = 7.0 / 48.0 * t3074 * t2118 * t19714 * t328 * t3079 + t6802 * t2397 / 24.0 - 7.0 / 24.0 * t19722 - t2388 * t6135 / 4.0 + 7.0 / 24.0 * t19726 - 7.0 / 4.0 * t19728 - 7.0 / 36.0 * t19731 + 35.0 / 72.0 * t19735 + t822 * t19738 * t833 / 96.0 + t6112 * t2397 / 24.0 - 7.0 / 48.0 * t3074 * t4395 * t19745 * t2362 - 7.0 / 48.0 * t2382 * t6158 * t19751 * t2362 - t6793 * t19759 / 2.0 + t19608 * t19764 / 16.0;
    (t19744, t19745, t19750, t19751, t19756, t19767)
}
