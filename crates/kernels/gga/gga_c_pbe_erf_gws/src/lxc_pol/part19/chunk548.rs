//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 548/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk548<F: Float>(t173: F, t2824: F, t184: F, t199: F, t1902: F, t1905: F, t1911: F, t1915: F, t1920: F, t1926: F, t1928: F, t2755: F, t2758: F, t2788: F, t2792: F, t2794: F, t2795: F, t2798: F, t2802: F, t2806: F, t2808: F, t2818: F) -> (F, F, F, F) {
    let t2825 = t173 * t2824;
    let t2826 = t2825 * t184;
    let t2828 = F::new(2.0) / F::new(15.0) * t2826 * t199;
    let t2829 = t2755 - t2758 - t2788 + t2792 - t2794 + t2795 + t2798 + t2802 + t1902 - t1905 + t1911 / F::new(3.0) + F::new(0.60777777777777777777e-1) * t1915 + t1920 + t1926 + t1928 - t2806 + t2808 + t2818 + t2828;
    (t2825, t2826, t2828, t2829)
}
