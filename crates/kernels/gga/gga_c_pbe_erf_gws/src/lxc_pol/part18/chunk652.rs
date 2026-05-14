//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 652/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk652<F: Float>(t343: F, t3824: F, t904: F, t916: F, t858: F, t867: F, t866: F, t2157: F, t2155: F, t339: F, t3703: F, t3717: F, t1130: F, t2181: F, t3154: F, t340: F, t3772: F, t870: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3825 = t3824 * t343;
    let t3826 = t904 * t3825;
    let t3827 = t916 * t3826;
    let t3831 = t858 * t3825;
    let t3832 = t867 * t3831;
    let t3834 = t866 * t3832 / 96.0;
    let t3835 = t3824 * t2157;
    let t3836 = t904 * t3835;
    let t3837 = t916 * t3836;
    let t3840 = t858 * t3835;
    let t3841 = t867 * t3840;
    let t3843 = t2155 * t3841 / 48.0;
    let t3848 = t339 * t3703;
    let t3851 = t339 * t3717;
    let t3854 = -t339 * t340 * t3772 + 6.0 * t1130 * t3154 - 12.0 * t2181 * t3848 + 3.0 * t3851 * t870;
    (t3825, t3827, t3831, t3832, t3834, t3835, t3837, t3840, t3841, t3843, t3848, t3851, t3854)
}
