//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1444/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1444<F: Float>(t41338: F, t41371: F, t923: F, t273: F, t2881: F, t2889: F, t2897: F, t41292: F, t41299: F, t41303: F, t41307: F, t41341: F, t41344: F, t41347: F, t41350: F, t41361: F, t41363: F, t41369: F) -> (F, F, F, F, F, F, F) {
    let t41372 = t41338 + t41371;
    let t41373 = t923 * t41372;
    let t41382 = F::powf(t273, -F::cast_from(0.25e1_f64));
    let t41383 = t2881 * t2881;
    let t41384 = t41382 * t41383;
    let t41386 = t2889 * t2889;
    let t41387 = t2897 * t41386;
    let t41389 = F::cast_from(0.98115555555555555555e-1_f64) * t41292 - F::cast_from(0.8585111111111111111e-1_f64) * t41299 - F::cast_from(0.82785e-1_f64) * t41303 + t41307 + F::cast_from(0.16504875e0_f64) * t41373 - F::cast_from(0.89459259259259259259e0_f64) * t41341 - F::cast_from(0.301925e0_f64) * t41344 - F::cast_from(0.72462e1_f64) * t41347 + F::cast_from(0.40256666666666666666e1_f64) * t41350 + F::cast_from(0.12524296296296296297e1_f64) * t41361 + F::cast_from(0.16102666666666666667e1_f64) * t41363 - F::cast_from(0.16102666666666666667e1_f64) * t41369 + F::cast_from(0.6189328125e-1_f64) * t41384 + F::cast_from(0.247573125e0_f64) * t41387;
    (t41372, t41373, t41383, t41384, t41386, t41387, t41389)
}
