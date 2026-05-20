//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1679/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1679<F: Float>(t2924: F, t6110: F, t6141: F, t41908: F, t63453: F, t63459: F, t63464: F, t77499: F, t77559: F, t77561: F, t88085: F, t88089: F, t88093: F, t88097: F) -> (F, F) {
    let t88510 = F::new(36.0) * t2924 * t6110 * t6141;
    let t88524 = F::cast_from(0.4566222222222222222e-1_f64) * t77559 - F::cast_from(0.13698666666666666667e0_f64) * t77561 + F::cast_from(0.25367901234567901233e-1_f64) * t77499 - F::cast_from(0.3044148148148148148e-1_f64) * t63453 + F::cast_from(0.9132444444444444444e-1_f64) * t63459 + t41908 + F::new(0.41096e0) * t88085 - F::new(0.61644e0) * t88089 + F::new(0.10274e0) * t88093 + F::cast_from(0.13698666666666666667e0_f64) * t88097 - F::cast_from(0.45662222222222222221e-1_f64) * t63464;
    (t88510, t88524)
}
