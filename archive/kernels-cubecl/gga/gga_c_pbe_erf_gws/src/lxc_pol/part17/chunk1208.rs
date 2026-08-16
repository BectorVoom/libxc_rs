//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1208/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1208<F: Float>(t13846: F, t4414: F, t13826: F, t840: F, t13837: F, t13822: F, t8801: F, t13972: F, t14118: F, t13772: F, t2367: F, t4002: F, t4474: F) -> (F, F, F, F, F, F, F) {
    let t51745 = t4414 * t13846;
    let t51756 = t840 * t13826;
    let t51758 = t4414 * t13837;
    let t51769 = t8801 * t13822;
    let t51771 = t13972 * t14118;
    let t51781 = t2367 * t13772;
    let t51788 = t4474 * t4002;
    (t51745, t51756, t51758, t51769, t51771, t51781, t51788)
}
