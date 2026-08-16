//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 728/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk728<F: Float>(t13329: F, t1450: F, t3795: F, t10500: F, t472: F, t1218: F, t338: F, t3923: F, t408: F, t3936: F, t3959: F, t1319: F, t4065: F, sigma0: F) -> (F, F, F, F, F, F, F, F) {
    let t13330 = t13329 * sigma0;
    let t13377 = t3795 * t1450;
    let t13399 = t10500 * t472;
    let t13400 = F::cast_from(0.73697530864197530862e-3_f64) * t13399;
    let t13435 = t1218 * t1218;
    let t13436 = F::cast_from(1.0_f64) / t13435;
    let t13437 = t338 * t13436;
    let t13440 = F::cast_from(1.0_f64) / t3923 / t408;
    let t13472 = t3936 * t3959;
    let t13485 = t4065 * t1319;
    (t13330, t13377, t13399, t13400, t13437, t13440, t13472, t13485)
}
