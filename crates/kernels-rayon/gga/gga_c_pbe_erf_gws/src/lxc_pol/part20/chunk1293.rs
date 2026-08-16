//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1293/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1293(t11889: f64, t14637: f64, t3974: f64, t3990: f64, t14001: f64, t3744: f64, t1161: f64, t1193: f64, t14576: f64, t14710: f64, t2409: f64, t3037: f64, t3066: f64, t3067: f64, t3207: f64, t34773: f64, t353: f64, t35566: f64, t53419: f64, t53425: f64, t53790: f64, t56431: f64, t56434: f64, t56439: f64, t56442: f64, t56445: f64, t56452: f64, t56456: f64, t56460: f64, t6793: f64, t859: f64, t8629: f64, t8793: f64) -> f64 {
    let t56474 = t14637 * t3990 * t3974 * t11889;
    let t56476 = t14001 * t3744;
    let t56480 = -t3207 * t35566 * t14710 / 8.0_f64 - t56431 / 1536.0_f64 - 5.0_f64 / 384.0_f64 * t56434 + t56439 / 1536.0_f64 + t56442 / 384.0_f64 + t6793 * t56445 / 24.0_f64 + t8793 * t53419 / 24.0_f64 + t6793 * t56452 / 24.0_f64 + t6793 * t56456 / 24.0_f64 - t56460 / 768.0_f64 - t34773 * t859 * t353 * t1193 * t3037 / 48.0_f64 - t53425 + t3066 * t2409 * t3067 * t14576 * t1161 / 24.0_f64 + 5.0_f64 / 384.0_f64 * t56474 - 7.0_f64 / 72.0_f64 * t56476 - t8629 * t53790 / 24.0_f64;
    t56480
}
