//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1079/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1079(t10458: f64, t874: f64, t352: f64, t1356: f64, t1364: f64, t39333: f64, t39341: f64, t39345: f64, t39364: f64, t42928: f64, t45355: f64, t45361: f64, t45363: f64, t45365: f64, t45367: f64, t45371: f64, t45374: f64, t45381: f64, t45385: f64, t45389: f64, t6400: f64, t699: f64) -> (f64, f64) {
    let t48431 = t874 * t10458;
    let t48432 = t48431 * t352;
    let t48450 = 0.85129199786595678799e-5_f64 * t45355 + 0.39914139006212695214e-1_f64 * t1356 * t48432 + 0.2553875993597870364e-4_f64 * t45361 + 0.10215503974391481456e-3_f64 * t45363 - 0.15323255961587222184e-3_f64 * t45365 - 0.5107751987195740728e-4_f64 * t45367 + 0.5107751987195740728e-4_f64 * t45371 + 0.325201597776800302e-2_f64 * t39333 - t42928 - 0.40911992481368012596e-1_f64 * t45374 - 0.47896966807455234256e0_f64 * t1364 * t699 * t6400 + 0.13680077012009379e-5_f64 * t39341 + 0.13680077012009379e-5_f64 * t39345 - t39364 + 0.212822999466489197e-4_f64 * t45381 - 0.212822999466489197e-4_f64 * t45385 - 0.3405167991463827152e-4_f64 * t45389;
    (t48432, t48450)
}
