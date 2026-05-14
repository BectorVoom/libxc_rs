//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1039/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1039<F: Float>(t1193: F, t3200: F, t338: F, t14001: F, t4130: F, t1192: F, t3306: F, t2409: F, t3067: F, t13953: F, t4135: F, t3294: F, t3975: F, t3972: F, t4182: F, t810: F) -> (F, F, F, F, F, F, F, F) {
    let t14742 = t338 * t3200 * t1193;
    let t14745 = t14001 * t4130;
    let t14747 = t1192 * t3306;
    let t14749 = t2409 * t3067 * t14747;
    let t14752 = t13953 * t4135;
    let t14754 = t3975 * t3294;
    let t14755 = t3972 * t14754;
    let t14757 = t4182 * t810;
    (t14742, t14745, t14747, t14749, t14752, t14754, t14755, t14757)
}
