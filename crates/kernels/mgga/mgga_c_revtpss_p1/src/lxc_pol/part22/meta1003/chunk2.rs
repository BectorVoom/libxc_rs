//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3420/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3420<F: Float>(t41330: F, t41332: F, t63474: F, t63476: F, t63478: F, t63480: F, t63482: F, t63485: F, t63488: F, t63491: F, t63494: F, t63497: F, t63500: F, t63503: F, t63505: F) -> F {
    let t64277 = -F::new(0.3529725e1) * t63474 - F::new(0.17648625e1) * t63476 - F::cast_from(0.157790625e0_f64) * t63478 + F::new(0.6311625e0) * t63480 + F::new(0.31558125e0) * t63482 - F::new(0.3529725e1) * t63485 + F::new(0.6311625e0) * t63488 - F::cast_from(0.6618234375e1_f64) * t63491 + F::cast_from(0.264729375e1_f64) * t63494 + F::cast_from(0.2366859375e0_f64) * t63497 - F::cast_from(0.157790625e0_f64) * t63500 + F::cast_from(0.83356000000000000001e0_f64) * t63503 + F::new(0.3529725e1) * t63505 - F::cast_from(0.22954444444444444444e0_f64) * t41330 - F::cast_from(0.15302962962962962963e0_f64) * t41332;
    t64277
}
