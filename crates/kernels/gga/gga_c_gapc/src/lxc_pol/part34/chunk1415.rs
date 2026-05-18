//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1415/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1415<F: Float>(t35160: F, t35162: F, t35149: F, t37210: F, t37211: F, t37212: F, t37213: F, t37214: F, t37216: F, t37217: F, t37218: F, t35169: F) -> (F, F) {
    let t37219 = F::new(0.33816362383187442026e-5) * t35160;
    let t37220 = F::new(0.80192315782160920384e-6) * t35162;
    let t37221 = -t37210 - t37211 - t37212 - t37213 + t37214 - F::new(0.64456181686737100543e-8) * t35149 + t37216 + t37217 + t37218 + t37219 - t37220;
    let t37223 = F::new(0.11984097313886885523e-6) * t35169;
    (t37221, t37223)
}
