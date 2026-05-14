//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 163/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk163<F: Float>(t436: F, t568: F, t120: F, t174: F, t344: F, t366: F, t371: F, t384: F, t385: F, t426: F, t434: F, t459: F, t466: F, t469: F, t473: F, t477: F, t508: F, t518: F, t523: F, t526: F) -> (F, F) {
    let t569 = t436 * t568;
    let t572 = t344 + t366 - t371 - t384 - 0.17379648562707520765e-2 * t385 * t174 + 0.10427789137624512459e-2 * t426 * t174 - 0.10427789137624512459e-2 * t434 * t459 - 0.3475929712541504153e-4 * t466 * t469 + 0.61802030288987943842e-4 * t473 * t477 + 0.10427789137624512459e-2 * t120 * t508 + 0.50690641641230268898e-4 * t518 * t523 - 0.10427789137624512459e-2 * t526 * t569;
    (t569, t572)
}
