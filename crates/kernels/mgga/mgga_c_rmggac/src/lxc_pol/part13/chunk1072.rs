//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1072/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1072<F: Float>(t40332: F, t798: F, t9540: F, t4048: F, t40339: F, t40349: F, t40351: F, t40354: F, t40356: F, t2208: F, t26283: F, t26291: F, t28295: F, t35683: F, t35691: F, t40329: F, t40335: F, t40337: F, t40343: F, t40345: F, t40347: F) -> (F, F, F) {
    let t43375 = F::cast_from(0.58540737209111952978e0_f64) * t40332;
    let t43377 = t9540 * t798;
    let t43380 = t9540 * t4048;
    let t43385 = F::cast_from(0.11918087970123395032e-3_f64) * t40339;
    let t43390 = F::cast_from(0.39726959900411316772e-4_f64) * t40349;
    let t43391 = F::cast_from(0.11918087970123395032e-3_f64) * t40351;
    let t43392 = F::cast_from(0.11918087970123395032e-3_f64) * t40354;
    let t43393 = F::cast_from(0.39726959900411316772e-4_f64) * t40356;
    let t43394 = F::cast_from(0.638468998399467591e-4_f64) * t40329 + F::cast_from(0.11974241701863808564e0_f64) * t28295 * t2208 - t43375 - F::cast_from(0.19863479950205658386e-4_f64) * t35683 - F::cast_from(0.14369090042236570277e1_f64) * t26283 * t43377 - F::cast_from(0.71845450211182851384e0_f64) * t26291 * t43380 + F::cast_from(0.35922725105591425692e0_f64) * t40335 + F::cast_from(0.11974241701863808564e0_f64) * t40337 + t43385 + F::cast_from(0.40992351065071538965e-3_f64) * t35691 + F::cast_from(0.29810146462873361016e-2_f64) * t40343 + F::cast_from(0.2553875993597870364e-4_f64) * t40345 - F::cast_from(0.5107751987195740728e-4_f64) * t40347 + t43390 - t43391 + t43392 + t43393;
    (t43377, t43380, t43394)
}
