//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1318/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1318<F: Float>(t3225: F, t35834: F, t10153: F, t35751: F, t6182: F, t11683: F, t11687: F, t22442: F, t11698: F, t6178: F, t297: F, t825: F) -> (F, F, F, F, F) {
    let t35835 = t3225 * t35834;
    let t35838 = t10153 * t35751 * t6182;
    let t35841 = t11687 * t11683 * t22442;
    let t35843 = t6178 * t11698;
    let t35846 = t825 * t297;
    (t35835, t35838, t35841, t35843, t35846)
}
