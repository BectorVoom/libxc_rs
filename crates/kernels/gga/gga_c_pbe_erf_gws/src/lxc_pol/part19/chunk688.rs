//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 688/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk688<F: Float>(t3879: F, t867: F, t866: F, t3145: F, t2266: F, t2336: F, t3271: F, t3274: F, t3302: F, t3827: F, t3834: F, t3837: F, t3843: F, t3857: F, t3860: F, t3863: F, t3869: F, t3871: F, t3875: F, t902: F, t914: F, t929: F) -> (F, F, F, F) {
    let t3880 = t867 * t3879;
    let t3882 = t866 * t3880 / F::new(96.0);
    let t3883 = F::new(7.0) / F::new(144.0) * t3145;
    let t3885 = -t914 * t3827 / F::new(1536.0) + F::new(7.0) / F::new(576.0) * t3271 - t3834 + t2266 * t3837 / F::new(512.0) + t3843 - t914 * t3857 / F::new(1536.0) + t3860 + t902 * t3863 / F::new(1536.0) - t3869 - t929 * t3871 / F::new(768.0) + F::new(5.0) / F::new(768.0) * t929 * t3875 - F::new(7.0) / F::new(1152.0) * t3274 - t3882 + t2336 + t3883 + F::new(7.0) / F::new(1152.0) * t3302;
    (t3880, t3882, t3883, t3885)
}
