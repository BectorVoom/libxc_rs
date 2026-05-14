//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1065/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1065<F: Float>(t2118: F, t838: F, t14138: F, t822: F, t2232: F, t4386: F, t13872: F, t13953: F, t13930: F, t19906: F, t13846: F, t4414: F, t13826: F, t840: F, t13837: F, t13822: F, t8801: F) -> (F, F, F, F, F, F, F, F, F) {
    let t51717 = t2118 * t838;
    let t51719 = t822 * t51717 * t14138;
    let t51721 = t4386 * t2232;
    let t51724 = t13953 * t13872;
    let t51726 = t19906 * t13930;
    let t51745 = t4414 * t13846;
    let t51756 = t840 * t13826;
    let t51758 = t4414 * t13837;
    let t51769 = t8801 * t13822;
    (t51717, t51719, t51721, t51724, t51726, t51745, t51756, t51758, t51769)
}
