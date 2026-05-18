//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1184/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1184<F: Float>(t47910: F, t47914: F, t47916: F, t47918: F, t47920: F, t47922: F, t47926: F, t47928: F, t48043: F, t48044: F, t48045: F, t26308: F, t26314: F, t41334: F, t48046: F, t48049: F, t48050: F, t48052: F, t48056: F, t48059: F, t48060: F, t48062: F) -> (F, F) {
    let t48659 = -t47910 + t47914 + t47916 - t47918 - t47920 - t47922 - t47926 + t47928 - t48043 - t48044 + t48045;
    let t48663 = -t48046 + t48049 - t48050 - t48052 - t48056 + F::new(0.44134814814814814813e-2) * t26308 + F::new(16.0) * t26314 + t48059 + t48060 + F::new(0.43284165449459373508e0) * t41334 - t48062;
    (t48659, t48663)
}
