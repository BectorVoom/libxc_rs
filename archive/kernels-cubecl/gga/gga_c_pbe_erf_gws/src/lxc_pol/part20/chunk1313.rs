//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1313/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1313<F: Float>(t13984: F, t56112: F, t12041: F, t3037: F, t353: F, t376: F, t51580: F, t859: F, t12020: F, t13917: F, t13919: F, t11360: F, t3959: F) -> (F, F, F, F) {
    let t56793 = t56112 * t13984;
    let t56799 = t12041 * t51580 * t859 * t353 * t376 * t3037;
    let t56811 = t13917 * t13919 * t12020;
    let t56813 = t3959 * t11360;
    (t56793, t56799, t56811, t56813)
}
