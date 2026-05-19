//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 778/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk778<F: Float>(t9097: F, t9100: F, t9104: F, t9106: F, t9108: F, t9111: F, t9115: F, t9118: F, t9121: F, t9124: F, t9126: F, t9130: F, t9132: F) -> F {
    let t9134 = -F::cast_from(0.16908181191593721013e-4_f64) * t9097 + F::cast_from(0.14492726735651760868e-5_f64) * t9100 - F::cast_from(0.45775879823985672486e-6_f64) * t9104 + F::cast_from(0.16908181191593721013e-4_f64) * t9106 - F::cast_from(0.50680539737635041234e-4_f64) * t9108 - F::cast_from(0.50680539737635041234e-4_f64) * t9111 - F::cast_from(0.18758436440271560323e-8_f64) * t9115 + F::cast_from(0.12647289956446654818e-8_f64) * t9118 + F::cast_from(0.6487109086417285278e-2_f64) * t9121 - F::cast_from(0.13900948042322754167e-2_f64) * t9124 + F::cast_from(0.66340671383216596998e-6_f64) * t9126 + F::cast_from(0.10120768229166666667e-3_f64) * t9130 + F::cast_from(0.27801896084645508334e-2_f64) * t9132;
    t9134
}
