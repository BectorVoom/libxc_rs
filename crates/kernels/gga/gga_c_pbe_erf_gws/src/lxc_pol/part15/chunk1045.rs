//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1045/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1045<F: Float>(t9380: F, t9607: F, t8546: F, t944: F, t3327: F, t810: F, t1198: F, t21885: F, t14145: F, t945: F, t804: F, t8556: F, t13757: F, t2429: F, t14154: F, t321: F) -> (F, F, F, F, F, F, F, F, F) {
    let t38360 = t9607 * t9380;
    let t43260 = t8546 * t944;
    let t47184 = t3327 * t810;
    let t50818 = t1198 * t21885;
    let t50825 = t14145 * t945;
    let t50832 = t804 * t1198;
    let t50833 = t50832 * t8556;
    let t50835 = t2429 * t13757;
    let t50837 = t321 * t14154;
    (t38360, t43260, t47184, t50818, t50825, t50832, t50833, t50835, t50837)
}
