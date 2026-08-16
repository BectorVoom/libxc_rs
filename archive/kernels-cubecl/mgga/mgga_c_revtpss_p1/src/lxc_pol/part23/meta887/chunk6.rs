//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2807/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2807<F: Float>(t4321: F, t6049: F, t689: F, t4481: F, t63084: F, t18323: F, t23383: F, t2770: F, t40970: F, t40978: F, t50161: F, t50214: F, t50219: F, t50221: F, t50223: F, t50240: F, t61385: F, t61397: F, t61400: F, t61403: F, t61407: F, t865: F, t886: F) -> F {
    let t75998 = t689 * t4321 * t6049;
    let t76010 = t63084 * t4481;
    let t76012 = F::cast_from(0.16463622957338778996e-1_f64) * t61385 - F::cast_from(0.11853808529283920877e2_f64) * t50240 * t50161 * t18323 - F::cast_from(0.26019841438354088051e-2_f64) * t40970 - F::cast_from(0.32927245914677557992e-1_f64) * t75998 - F::cast_from(0.19637199382202157274e-3_f64) * t40978 - F::cast_from(0.13878983423218070567e-1_f64) * t50214 - t50219 - t50221 - t50223 - F::cast_from(0.39029762157531132074e-2_f64) * t61397 + F::cast_from(0.39029762157531132074e-2_f64) * t61400 - F::cast_from(0.32927245914677557992e-1_f64) * t61403 + F::cast_from(0.13170898365871023197e1_f64) * t865 * t2770 * t23383 * t886 + F::cast_from(0.69394917116090352834e-2_f64) * t61407 - F::cast_from(0.29272321618148349057e-1_f64) * t76010;
    t76012
}
