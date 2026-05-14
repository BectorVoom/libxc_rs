//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 754/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk754<F: Float>(t43: F, t50: F, t13069: F, t312: F, t6906: F, t1167: F, t321: F, t9772: F, t12917: F, t12919: F, t12921: F, t12923: F, t12925: F, t12927: F, zeta_threshold: F) -> (F, F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t13070 = t13069 * t312;
    let t13071 = 0.20538164420033333334e1 * t6906;
    let t13073 = t321 * t9772 * t1167;
    let t13079 = piecewise3(t44, 0.0, 8.0 / 27.0 * t12917 - 2.0 / 3.0 * t12919 + 2.0 / 3.0 * t12921);
    let t13084 = piecewise3(t51, 0.0, 8.0 / 27.0 * t12923 - 2.0 / 3.0 * t12925 + 2.0 / 3.0 * t12927);
    let t13086 = t13079 / 2.0 + t13084 / 2.0;
    (t13070, t13071, t13073, t13086)
}
