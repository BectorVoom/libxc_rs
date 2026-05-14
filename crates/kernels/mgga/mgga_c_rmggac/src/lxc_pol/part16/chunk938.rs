//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 938/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk938<F: Float>(t10458: F, t874: F, t352: F, t1356: F, t1364: F, t39333: F, t39341: F, t39345: F, t39364: F, t42928: F, t45355: F, t45361: F, t45363: F, t45365: F, t45367: F, t45371: F, t45374: F, t45381: F, t45385: F, t45389: F, t6400: F, t699: F) -> (F, F) {
    let t48431 = t874 * t10458;
    let t48432 = t48431 * t352;
    let t48450 = 0.85129199786595678799e-5 * t45355 + 0.39914139006212695214e-1 * t1356 * t48432 + 0.2553875993597870364e-4 * t45361 + 0.10215503974391481456e-3 * t45363 - 0.15323255961587222184e-3 * t45365 - 0.5107751987195740728e-4 * t45367 + 0.5107751987195740728e-4 * t45371 + 0.325201597776800302e-2 * t39333 - t42928 - 0.40911992481368012596e-1 * t45374 - 0.47896966807455234256e0 * t1364 * t699 * t6400 + 0.13680077012009379e-5 * t39341 + 0.13680077012009379e-5 * t39345 - t39364 + 0.212822999466489197e-4 * t45381 - 0.212822999466489197e-4 * t45385 - 0.3405167991463827152e-4 * t45389;
    (t48432, t48450)
}
