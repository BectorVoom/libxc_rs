//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 893/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk893<F: Float>(t209: F, t50939: F, t50949: F, t50953: F, t50958: F, t50962: F, t50966: F, t50977: F, t50979: F, t47107: F, t47114: F, t47120: F, t47784: F, t42934: F, t42937: F, t42940: F, t42943: F, t42948: F, t42954: F, t42961: F, t42963: F, t42967: F, t42970: F, t47587: F, t47594: F, t47597: F, t47602: F, t47605: F, t47607: F, t47610: F) -> (F, F, F, F, F, F) {
    let t50983 = (t50939 + t50949 + t50953 + t50958 + t50962 + t50966 + t50977 + t50979) * t209;
    let t50984 = 4.0 * t47107;
    let t50985 = 4.0 * t47114;
    let t50986 = 4.0 * t47120;
    let t50987 = 12.0 * t47784;
    let t51000 = -0.17090058289204942852e-2 * t47587 - t42934 - t42937 - t42940 + t42943 + t42948 - t42954 - t42961 + 0.7690526230142224284e-2 * t42963 + 0.64087718584518535698e-3 * t47594 - 0.3845263115071112142e-2 * t42967 - 0.1281754371690370714e-2 * t42970 - 0.64087718584518535698e-3 * t47597 - t47602 + t47605 - 0.1922631557535556071e-2 * t47607 + 0.1281754371690370714e-2 * t47610;
    (t50983, t50984, t50985, t50986, t50987, t51000)
}
