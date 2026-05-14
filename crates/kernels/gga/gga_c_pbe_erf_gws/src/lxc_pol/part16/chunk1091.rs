//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1091/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1091<F: Float>(t13792: F, t8790: F, t13776: F, t37214: F, t3975: F, t1113: F, t2182: F, t51555: F, t824: F, t13917: F, t51678: F, t9337: F, t3959: F, t8812: F, t13796: F, t14423: F, t2190: F, t3989: F) -> (F, F, F, F, F, F) {
    let t53517 = t13792 * t8790;
    let t53520 = t13776 * t3975 * t37214;
    let t53526 = t51555 * t3975 * t1113 * t824 * t2182;
    let t53529 = t13917 * t51678 * t9337;
    let t53531 = t3959 * t8812;
    let t53537 = t3989 * t13796 * t14423 * t2190;
    (t53517, t53520, t53526, t53529, t53531, t53537)
}
