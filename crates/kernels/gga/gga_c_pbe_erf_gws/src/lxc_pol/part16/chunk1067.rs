//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1067/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1067<F: Float>(t14260: F, t4414: F, t14322: F, t1206: F, t2182: F, t353: F, t8599: F, t14291: F, t9270: F, t22509: F, t4099: F, t14266: F, t14311: F, t2367: F, t4083: F, t6745: F) -> (F, F, F, F, F, F, F, F) {
    let t52199 = t4414 * t14260;
    let t52217 = t4414 * t14322;
    let t52241 = t8599 * t353 * t1206 * t2182;
    let t52249 = t9270 * t14291;
    let t52251 = t22509 * t4099;
    let t52263 = t9270 * t14266;
    let t52266 = t2367 * t14311;
    let t52270 = t6745 * t4083;
    (t52199, t52217, t52241, t52249, t52251, t52263, t52266, t52270)
}
