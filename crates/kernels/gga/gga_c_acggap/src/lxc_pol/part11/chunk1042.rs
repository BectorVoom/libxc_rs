//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1042/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1042<F: Float>(t36207: F, t1439: F, t1992: F, t30780: F, t1460: F, t30148: F, t2035: F, t7323: F, t16314: F, t336: F, t570: F, t4264: F, t7436: F, t142: F, t3706: F, t1017: F, t2060: F, t2288: F) -> (F, F, F, F, F, F, F, F) {
    let t36208 = 0.916875e-1 * t36207;
    let t36209 = t1992 * t1439;
    let t36210 = t30780 * t36209;
    let t36211 = 0.916875e-1 * t36210;
    let t36213 = t30148 * t1460;
    let t36214 = t2035 * t7323 * t36213;
    let t36215 = 0.916875e-1 * t36214;
    let t36217 = t570 * t336 * t16314;
    let t36220 = t7436 * t4264;
    let t36222 = t142 * t3706;
    let t36225 = t2060 * t36222 * t2288 * t1017;
    (t36208, t36209, t36211, t36213, t36215, t36217, t36220, t36225)
}
