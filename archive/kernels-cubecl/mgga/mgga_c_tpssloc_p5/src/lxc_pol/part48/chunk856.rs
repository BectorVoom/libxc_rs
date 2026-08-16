//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 856/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk856<F: Float>(t25373: F, t31448: F, t1914: F, t606: F, t1877: F, t193: F, t202: F, t24339: F, t24344: F, t2522: F, t31429: F, t31434: F, t31441: F, t6665: F, t7114: F, t776: F, t8566: F, t868: F, t870: F) -> (F, F, F) {
    let t31449 = t25373 * t31448;
    let t31451 = t606 * t1914;
    let t31477 = t193 * t202 * t31429 * t870 - t1877 * t1914 * t24339 + F::cast_from(2.0_f64) * t1877 * t24344 * t31448 - t1877 * t31434 * t868 - t1877 * t6665 * t7114 - F::cast_from(3.0_f64) * t2522 * t31441 * t7114 + F::cast_from(3.0_f64) * t2522 * t776 * t8566;
    (t31449, t31451, t31477)
}
