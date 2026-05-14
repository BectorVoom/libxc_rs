//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 985/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk985<F: Float>(t139212: F, t139224: F, t27147: F, t32899: F, t139213: F, t27158: F, t631: F, t95262: F, t147647: F, t23667: F, t5899: F, t34808: F, t379: F, t139352: F, t27081: F, t32962: F) -> (F, F, F, F, F, F) {
    let t148270 = t139212 * t139224 * t32899 * t27147;
    let t148275 = t95262 * t631 * t139213 * t32899 * t27158;
    let t148278 = t5899 * t23667 * t147647;
    let t148280 = t34808 * t379;
    let t148282 = t139212 * t139352 * t148280;
    let t148284 = t32962 * t27081;
    (t148270, t148275, t148278, t148280, t148282, t148284)
}
