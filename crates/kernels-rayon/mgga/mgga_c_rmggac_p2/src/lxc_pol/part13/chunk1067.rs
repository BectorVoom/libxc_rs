//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1067/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1067(t40062: f64, t40075: f64, t40084: f64, t40086: f64, t40088: f64, t37768: f64, t40068: f64, t40073: f64, t40082: f64, t40093: f64, t40098: f64, t40102: f64, t40106: f64, t40110: f64, t40112: f64, t40114: f64, t40116: f64) -> f64 {
    let t43267 = 0.39726959900411316772e-4_f64 * t40062;
    let t43270 = 0.49658699875514145965e-4_f64 * t40075;
    let t43272 = 0.3842256877732895568e-2_f64 * t40084;
    let t43273 = 0.3842256877732895568e-2_f64 * t40086;
    let t43274 = 0.3842256877732895568e-2_f64 * t40088;
    let t43283 = -t43267 + 0.1064114997332445985e-4_f64 * t40068 - 0.23942587439980034662e-4_f64 * t40073 - t43270 - t37768 + 0.5107751987195740728e-4_f64 * t40082 + t43272 + t43273 + t43274 + 0.5107751987195740728e-4_f64 * t40093 - 0.212822999466489197e-4_f64 * t40098 - 0.5107751987195740728e-4_f64 * t40102 + 0.61293023846348888736e-3_f64 * t40106 + 0.15323255961587222184e-3_f64 * t40110 - 0.2553875993597870364e-4_f64 * t40112 - 0.1702583995731913576e-4_f64 * t40114 - 0.5107751987195740728e-4_f64 * t40116;
    t43283
}
