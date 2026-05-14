//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 871/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk871<F: Float>(t10848: F, t593: F, t2615: F, t2643: F, t3421: F, t610: F, t5543: F, t587: F, t10778: F, t2559: F, t10837: F, t10838: F, t10840: F, t10841: F, t10845: F, t10847: F, t5359: F, t7617: F, t7619: F, t7623: F, t7665: F, t7668: F, t7672: F) -> (F, F, F, F, F) {
    let t10850 = 4.0 / 45.0 * t10848 * t593;
    let t10851 = t2615 * t2643;
    let t10852 = 16.0 / 135.0 * t10851;
    let t10853 = t3421 * t610;
    let t10854 = t5543 * t10853;
    let t10856 = 4.0 / 27.0 * t587 * t10854;
    let t10857 = t2559 * t10778;
    let t10859 = 8.0 / 9.0 * t587 * t10857;
    let t10860 = -t10837 + t7617 + t7619 + t7623 - t10838 + t5359 - t10840 + 2.0 / 9.0 * t10841 - t7665 - t7668 + t7672 + t10845 - t10847 + t10850 + t10852 - t10856 - t10859;
    (t10850, t10852, t10856, t10859, t10860)
}
