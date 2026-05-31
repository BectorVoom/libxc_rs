//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1945/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1945<F: Float>(t26205: F, t7709: F, t101129: F, t101132: F, t101139: F, t101337: F, t101782: F, t101783: F, t101785: F, t101790: F, t2048: F, t25159: F, t26175: F, t28116: F, t28119: F, t7352: F, t7706: F, t95310: F) -> F {
    let t101793 = t7709 * t26205;
    let t101805 = F::cast_from(10.0_f64) * t26175 * t101337 + t101782 - F::cast_from(440.0_f64) / F::cast_from(27.0_f64) * t101783 + F::cast_from(10.0_f64) * t101785 * t25159 + t101790 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t95310 * t7706 - F::cast_from(176.0_f64) / F::cast_from(27.0_f64) * t101793 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t101129 * t2048 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t101132 * t2048 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t28116 * t7352 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t101139 * t2048 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t28119 * t7352;
    t101805
}
