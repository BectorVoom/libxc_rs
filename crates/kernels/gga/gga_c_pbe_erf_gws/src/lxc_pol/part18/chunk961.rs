//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 961/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk961<F: Float>(t10843: F, t593: F, t2637: F, t7130: F, t3487: F, t586: F, t2615: F, t2643: F, t3421: F, t610: F, t5543: F, t587: F) -> (F, F, F, F, F) {
    let t10845 = F::new(8.0) / F::new(45.0) * t10843 * t593;
    let t10847 = F::new(8.0) / F::new(15.0) * t7130 * t2637;
    let t10848 = t3487 * t586;
    let t10850 = F::new(4.0) / F::new(45.0) * t10848 * t593;
    let t10851 = t2615 * t2643;
    let t10852 = F::new(16.0) / F::new(135.0) * t10851;
    let t10853 = t3421 * t610;
    let t10854 = t5543 * t10853;
    let t10856 = F::new(4.0) / F::new(27.0) * t587 * t10854;
    (t10845, t10847, t10850, t10852, t10856)
}
