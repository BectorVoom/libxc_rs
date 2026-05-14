//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1103/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1103<F: Float>(t4138: F, t50948: F, t14766: F, t833: F, t8945: F, t1114: F, t51922: F, t14138: F, t14733: F, t51042: F, t14001: F, t3214: F, t3950: F, t850: F, t9170: F, t13944: F, t2503: F) -> (F, F, F, F, F, F, F) {
    let t53886 = t50948 * t4138;
    let t53889 = t8945 * t14766 * t833;
    let t53891 = t1114 * t51922;
    let t53892 = t53891 * t14138;
    let t53894 = t14733 * t51042;
    let t53896 = t14001 * t3214;
    let t53904 = t850 * t9170 * t3950 * t833;
    let t53906 = t13944 * t2503;
    (t53886, t53889, t53892, t53894, t53896, t53904, t53906)
}
