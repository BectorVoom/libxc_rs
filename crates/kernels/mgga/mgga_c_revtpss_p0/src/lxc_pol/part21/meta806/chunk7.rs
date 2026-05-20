//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2941/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2941<F: Float>(t11977: F, t4820: F, t1042: F, t1063: F, t11859: F, t15834: F, t16076: F, t16208: F, t1675: F, t19634: F, t3117: F, t3188: F, t42195: F, t42227: F, t42230: F, t42232: F, t4806: F, t53450: F, t53459: F, t53464: F, t53473: F, t53474: F) -> F {
    let t53479 = t11977 * t4820;
    let t53490 = F::cast_from(0.14291339372689912324e-2_f64) * t3188 * t15834 + F::cast_from(0.71456696863449561621e-3_f64) * t1063 * t1042 * t4806 * t53459 + F::cast_from(0.71456696863449561621e-3_f64) * t1063 * t1042 * t4806 * t53464 + F::cast_from(0.19055119163586549765e-2_f64) * t1063 * t1042 * t16208 * t53450 + F::cast_from(0.23289590088828005269e-2_f64) * t1063 * t1042 * t53473 * t53474 - F::cast_from(0.45732285992607719436e-2_f64) * t53479 + F::cast_from(0.42874018118069736972e-3_f64) * t42227 + F::cast_from(0.14291339372689912324e-3_f64) * t42230 + F::cast_from(0.42874018118069736972e-3_f64) * t42232 - F::cast_from(0.22866142996303859718e-2_f64) * t42195 * t1675 - F::cast_from(0.12862205435420921092e-2_f64) * t11859 * t3117 * t16076 * t19634;
    t53490
}
