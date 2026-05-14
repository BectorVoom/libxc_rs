//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 805/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk805<F: Float>(t42967: F, t10736: F, t29277: F, t7064: F, t10635: F, t2554: F, t1841: F, t3487: F, t734: F, t9641: F, t123: F, t1843: F, t42921: F, t42925: F, t42931: F, t42934: F, t42937: F, t42940: F, t42943: F, t42948: F, t42951: F, t42954: F, t42956: F, t42961: F, t42964: F) -> (F,) {
    let t42968 = 0.38452631150711121417e-2 * t42967;
    let t42970 = t7064 * t29277 * t10736;
    let t42971 = 0.12817543716903707139e-2 * t42970;
    let t42973 = t7064 * t10635 * t2554;
    let t42974 = 0.64087718584518535698e-3 * t42973;
    let t42978 = 0.85450291446024714263e-3 * t1841 * t9641 * t3487 * t734;
    let t42979 = 0.85450291446024714263e-3 * t1841 * t1843 * t42921 - 0.85450291446024714263e-3 * t1841 * t42925 * t123 * t734 - 0.64087718584518535698e-3 * t42931 - t42934 - t42937 - t42940 + t42943 + t42948 - 0.1922631557535556071e-2 * t42951 - t42954 + 0.1281754371690370714e-2 * t42956 - t42961 + t42964 - t42968 - t42971 - t42974 - t42978;
    (t42979,)
}
