//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 163/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk163<F: Float>(t436: F, t568: F, t120: F, t174: F, t344: F, t366: F, t371: F, t384: F, t385: F, t426: F, t434: F, t459: F, t466: F, t469: F, t473: F, t477: F, t508: F, t518: F, t523: F, t526: F) -> (F, F) {
    let t569 = t436 * t568;
    let t572 = t344 + t366 - t371 - t384 - F::cast_from(0.17379648562707520765e-2_f64) * t385 * t174 + F::cast_from(0.10427789137624512459e-2_f64) * t426 * t174 - F::cast_from(0.10427789137624512459e-2_f64) * t434 * t459 - F::cast_from(0.3475929712541504153e-4_f64) * t466 * t469 + F::cast_from(0.61802030288987943842e-4_f64) * t473 * t477 + F::cast_from(0.10427789137624512459e-2_f64) * t120 * t508 + F::cast_from(0.50690641641230268898e-4_f64) * t518 * t523 - F::cast_from(0.10427789137624512459e-2_f64) * t526 * t569;
    (t569, t572)
}
