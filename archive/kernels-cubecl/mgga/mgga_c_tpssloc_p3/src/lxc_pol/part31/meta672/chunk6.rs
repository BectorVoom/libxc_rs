//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2019/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2019<F: Float>(t93633: F, t93636: F, t97202: F, t97204: F, t97206: F, t97208: F, t97210: F, t97212: F, t97214: F, t97217: F, t97219: F, t97221: F, t97223: F, t97225: F, t97227: F, t97229: F, t97231: F) -> F {
    let t102647 = -t93633 + t93636 + t97202 / F::cast_from(128.0_f64) + t97204 / F::cast_from(384.0_f64) + t97206 / F::cast_from(96.0_f64) + t97208 / F::cast_from(96.0_f64) - t97210 / F::cast_from(384.0_f64) - t97212 / F::cast_from(768.0_f64) + t97214 / F::cast_from(96.0_f64) + t97217 / F::cast_from(192.0_f64) - F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t97219 - t97221 / F::cast_from(768.0_f64) - F::cast_from(5.0_f64) / F::cast_from(96.0_f64) * t97223 + t97225 / F::cast_from(96.0_f64) - t97227 / F::cast_from(384.0_f64) + t97229 / F::cast_from(96.0_f64) + t97231 / F::cast_from(192.0_f64);
    t102647
}
