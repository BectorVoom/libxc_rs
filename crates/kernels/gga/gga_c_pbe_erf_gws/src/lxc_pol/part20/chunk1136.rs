//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1136/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1136<F: Float>(t1161: F, t353: F, t53614: F, t859: F, t1144: F, t14418: F, t11450: F, t13917: F, t51544: F, t11889: F, t14637: F, t3974: F, t3990: F, t14001: F, t3744: F, t1193: F, t14576: F, t14710: F, t2409: F, t3037: F, t3066: F, t3067: F, t3207: F, t34773: F, t35566: F, t53419: F, t53425: F, t53790: F, t56431: F, t56434: F, t56439: F, t56442: F, t56445: F, t6793: F, t8629: F, t8793: F) -> (F,) {
    let t56452 = t859 * t353 * t53614 * t1161;
    let t56456 = t859 * t1144 * t14418;
    let t56460 = t13917 * t51544 * t11450;
    let t56474 = t14637 * t3990 * t3974 * t11889;
    let t56476 = t14001 * t3744;
    let t56480 = -t3207 * t35566 * t14710 / 8.0 - t56431 / 1536.0 - 5.0 / 384.0 * t56434 + t56439 / 1536.0 + t56442 / 384.0 + t6793 * t56445 / 24.0 + t8793 * t53419 / 24.0 + t6793 * t56452 / 24.0 + t6793 * t56456 / 24.0 - t56460 / 768.0 - t34773 * t859 * t353 * t1193 * t3037 / 48.0 - t53425 + t3066 * t2409 * t3067 * t14576 * t1161 / 24.0 + 5.0 / 384.0 * t56474 - 7.0 / 72.0 * t56476 - t8629 * t53790 / 24.0;
    (t56480,)
}
