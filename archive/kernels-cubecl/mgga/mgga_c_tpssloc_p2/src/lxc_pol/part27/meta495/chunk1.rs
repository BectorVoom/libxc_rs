//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1883/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1883<F: Float>(t25097: F, t815: F, t23097: F, t23096: F, t23106: F, t23108: F, t23114: F, t23119: F, t25085: F, t25087: F, t25089: F, t25091: F, t25095: F) -> (F, F) {
    let t25098 = t815 * t25097;
    let t25099 = t23097 * t25098;
    let t25103 = t23096 - t23106 + t25085 / F::cast_from(768.0_f64) + t25087 / F::cast_from(384.0_f64) - t25089 / F::cast_from(1536.0_f64) + t25091 / F::cast_from(384.0_f64) + F::cast_from(0.40372756094140390854e-3_f64) * t25095 + t23108 + F::cast_from(0.12111826828242117256e-2_f64) * t25099 + F::cast_from(0.33643963411783659045e-4_f64) * t23114 - F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t23119;
    (t25098, t25103)
}
