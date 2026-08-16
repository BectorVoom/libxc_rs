//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 620/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk620<F: Float>(t9097: F, t9100: F, t9108: F, t9111: F, t9113: F, t9115: F, t10209: F, t3526: F, t471: F, t64: F) -> (F, F) {
    let t11210 = -F::cast_from(21.0_f64) / F::cast_from(128.0_f64) * t9097 + F::cast_from(147.0_f64) / F::cast_from(4096.0_f64) * t9100 - F::cast_from(63.0_f64) / F::cast_from(262144.0_f64) * t9108 + F::cast_from(21.0_f64) / F::cast_from(262144.0_f64) * t9111 - F::cast_from(49.0_f64) / F::cast_from(4096.0_f64) * t9113 + F::cast_from(7.0_f64) / F::cast_from(128.0_f64) * t9115;
    let t11218 = t11210 * t471 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t3526 * t64 + t10209 - F::cast_from(7.0_f64) / F::cast_from(128.0_f64) * t9097 + F::cast_from(21.0_f64) / F::cast_from(4096.0_f64) * t9100 - F::cast_from(7.0_f64) / F::cast_from(4096.0_f64) * t9113 + F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t9115;
    (t11210, t11218)
}
