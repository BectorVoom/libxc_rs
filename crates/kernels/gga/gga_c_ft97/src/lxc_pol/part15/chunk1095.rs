//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1095/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1095<F: Float>(t17630: F, t4431: F, t1073: F, t12116: F, t12122: F, t20027: F, t20035: F, t2265: F, t2266: F, t4462: F, t48117: F, t4883: F, t75994: F, t76056: F, t76062: F, t76101: F, t76126: F, t76128: F, t76130: F, t8654: F) -> F {
    let t87843 = t17630 * t4431;
    let t87868 = -F::cast_from(8.0_f64) * t75994 - F::cast_from(160.0_f64) / F::cast_from(81.0_f64) * t48117 + F::cast_from(8.0_f64) * t2265 * t12116 * t87843 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t2265 * t12122 * t87843 - F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t76056 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t76062 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t76101 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t76126 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t76128 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t76130 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t2265 * t8654 * t20027 * t1073 - F::cast_from(2.0_f64) * t2265 * t2266 * t4462 * t4883 - F::cast_from(8.0_f64) * t2265 * t2266 * t20035 * t1073;
    t87868
}
