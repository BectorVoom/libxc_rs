//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 938/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk938<F: Float>(t40332: F, t798: F, t9540: F, t4048: F, t40339: F, t40349: F, t40351: F, t40354: F, t40356: F, t2208: F, t26283: F, t26291: F, t28295: F, t35683: F, t35691: F, t40329: F, t40335: F, t40337: F, t40343: F, t40345: F, t40347: F) -> (F, F, F) {
    let t43375 = 0.58540737209111952978e0 * t40332;
    let t43377 = t9540 * t798;
    let t43380 = t9540 * t4048;
    let t43385 = 0.11918087970123395032e-3 * t40339;
    let t43390 = 0.39726959900411316772e-4 * t40349;
    let t43391 = 0.11918087970123395032e-3 * t40351;
    let t43392 = 0.11918087970123395032e-3 * t40354;
    let t43393 = 0.39726959900411316772e-4 * t40356;
    let t43394 = 0.638468998399467591e-4 * t40329 + 0.11974241701863808564e0 * t28295 * t2208 - t43375 - 0.19863479950205658386e-4 * t35683 - 0.14369090042236570277e1 * t26283 * t43377 - 0.71845450211182851384e0 * t26291 * t43380 + 0.35922725105591425692e0 * t40335 + 0.11974241701863808564e0 * t40337 + t43385 + 0.40992351065071538965e-3 * t35691 + 0.29810146462873361016e-2 * t40343 + 0.2553875993597870364e-4 * t40345 - 0.5107751987195740728e-4 * t40347 + t43390 - t43391 + t43392 + t43393;
    (t43377, t43380, t43394)
}
