//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1210/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1210<F: Float>(t13865: F, t51666: F, t14001: F, t2412: F, t1176: F, t1180: F, t6589: F, t13987: F, t894: F, t13855: F, t13953: F, t1193: F, t2182: F, t353: F, t8599: F) -> (F, F, F, F, F, F) {
    let t51829 = t51666 * t13865;
    let t51864 = t14001 * t2412;
    let t51869 = t1176 * t6589 * t1180;
    let t51870 = F::new(595.0) / F::new(10368.0) * t51869;
    let t51877 = t13987 * t894;
    let t51881 = t13953 * t13855;
    let t51890 = t8599 * t353 * t1193 * t2182;
    (t51829, t51864, t51870, t51877, t51881, t51890)
}
