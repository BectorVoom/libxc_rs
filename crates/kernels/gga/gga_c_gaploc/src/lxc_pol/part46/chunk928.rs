//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 928/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk928<F: Float>(t123: F, t1841: F, t1843: F, t42921: F, t42925: F, t42931: F, t42934: F, t42937: F, t42940: F, t42943: F, t42948: F, t42951: F, t42954: F, t42956: F, t42961: F, t42964: F, t42968: F, t42971: F, t42974: F, t42978: F, t734: F) -> F {
    let t42979 = F::new(0.85450291446024714263e-3) * t1841 * t1843 * t42921 - F::new(0.85450291446024714263e-3) * t1841 * t42925 * t123 * t734 - F::new(0.64087718584518535698e-3) * t42931 - t42934 - t42937 - t42940 + t42943 + t42948 - F::new(0.1922631557535556071e-2) * t42951 - t42954 + F::new(0.1281754371690370714e-2) * t42956 - t42961 + t42964 - t42968 - t42971 - t42974 - t42978;
    t42979
}
