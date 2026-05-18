//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1356/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1356<F: Float>(t14688: F, t2503: F, t13796: F, t13859: F, t2171: F, t56296: F, t3959: F, t9928: F, t14121: F, t9948: F, t15282: F, t51666: F) -> (F, F, F, F, F) {
    let t57326 = t14688 * t2503;
    let t57330 = t13859 * t13796 * t56296 * t2171;
    let t57332 = t3959 * t9928;
    let t57334 = t14121 * t9948;
    let t57338 = t51666 * t15282;
    (t57326, t57330, t57332, t57334, t57338)
}
