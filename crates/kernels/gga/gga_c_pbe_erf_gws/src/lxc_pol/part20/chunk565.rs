//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 565/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk565<F: Float>(t2014: F, t2015: F, t2745: F, t2746: F, t2748: F, t2751: F, t2755: F, t2758: F, t2788: F, t2792: F, t2794: F, t2795: F, t2798: F, t2802: F, t2806: F, t2808: F, t2818: F, t2828: F) -> F {
    let t2977 = -t2745 + t2746 + t2748 - t2751 + t2755 - t2758 - t2788 + t2792 - t2794 + t2795 + t2014 + F::new(4.0) / F::new(3.0) * t2015 + t2798 + t2802 - t2806 + t2808 + t2818 + t2828;
    t2977
}
