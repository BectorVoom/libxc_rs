//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 731/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk731<F: Float>(t611: F, t9128: F, t3085: F, t3160: F, t608: F, t9097: F, t9100: F, t9104: F, t9106: F, t9108: F, t9111: F, t9115: F, t9118: F, t9121: F, t9124: F, t9126: F) -> (F, F, F) {
    let t9129 = t611 * t9128;
    let t9130 = t9129 * t3085;
    let t9132 = t3160 * t608;
    let t9134 = -0.16908181191593721013e-4 * t9097 + 0.14492726735651760868e-5 * t9100 - 0.45775879823985672486e-6 * t9104 + 0.16908181191593721013e-4 * t9106 - 0.50680539737635041234e-4 * t9108 - 0.50680539737635041234e-4 * t9111 - 0.18758436440271560323e-8 * t9115 + 0.12647289956446654818e-8 * t9118 + 0.6487109086417285278e-2 * t9121 - 0.13900948042322754167e-2 * t9124 + 0.66340671383216596998e-6 * t9126 + 0.10120768229166666667e-3 * t9130 + 0.27801896084645508334e-2 * t9132;
    (t9130, t9132, t9134)
}
