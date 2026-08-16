//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1243/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1243<F: Float>(t14469: F, t50936: F, t3972: F, t3975: F, t9410: F, t13793: F, t53229: F, t13792: F, t8790: F, t13776: F, t37214: F, t1113: F, t2182: F, t51555: F, t824: F) -> (F, F, F, F, F, F) {
    let t53510 = t50936 * t14469;
    let t53513 = t3972 * t3975 * t9410;
    let t53515 = t53229 * t13793;
    let t53517 = t13792 * t8790;
    let t53520 = t13776 * t3975 * t37214;
    let t53526 = t51555 * t3975 * t1113 * t824 * t2182;
    (t53510, t53513, t53515, t53517, t53520, t53526)
}
