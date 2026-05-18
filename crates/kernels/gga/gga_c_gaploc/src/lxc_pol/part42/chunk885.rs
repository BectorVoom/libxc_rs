//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 885/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk885<F: Float>(t2033: F, t2365: F, t35451: F, t11784: F, t2679: F, t9800: F, t2617: F, t3626: F, t7810: F, t3614: F, t5241: F, t9805: F) -> (F, F, F, F) {
    let t45819 = t2033 * t2365 * t35451;
    let t45820 = F::new(0.44688112439813033337e-1) * t45819;
    let t45822 = t9800 * t11784 * t2679;
    let t45823 = F::new(0.9585731488480187419e0) * t45822;
    let t45826 = t7810 * t3626 * t2617;
    let t45828 = t5241 * t3614;
    let t45830 = t9805 * t45828 * t2679;
    (t45820, t45823, t45826, t45830)
}
