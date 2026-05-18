//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1241/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1241<F: Float>(t288: F, t7835: F, t2586: F, t8057: F, t940: F, t8149: F, t8153: F, t530: F, t864: F, t2721: F, t2724: F, t2723: F, t7299: F) -> (F, F, F, F, F) {
    let t25610 = t288 * t7835;
    let t25618 = t940 * t2586 * t8057;
    let t25620 = t8149 * t8153;
    let t25622 = t530 * t864;
    let t25624 = t2721 * t25622 * t2724;
    let t25633 = t2723 * t7299;
    (t25610, t25618, t25620, t25624, t25633)
}
