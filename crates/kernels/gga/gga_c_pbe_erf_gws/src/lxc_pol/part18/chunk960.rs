//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 960/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk960<F: Float>(t10782: F, t10832: F, t598: F, t186: F, t185: F, t5355: F, t3488: F, t612: F, t3584: F, t723: F, t3398: F, t586: F) -> (F, F, F, F, F) {
    let t10833 = t10782 + t10832;
    let t10834 = t598 * t10833;
    let t10835 = t186 * t10834;
    let t10837 = F::new(2.0) / F::new(15.0) * t185 * t10835;
    let t10838 = F::new(4.0) / F::new(135.0) * t5355;
    let t10840 = F::new(2.0) / F::new(15.0) * t3488 * t612;
    let t10841 = t3584 * t723;
    let t10843 = t3398 * t586;
    (t10837, t10838, t10840, t10841, t10843)
}
