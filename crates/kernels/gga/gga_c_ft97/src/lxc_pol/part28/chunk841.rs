//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 841/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk841<F: Float>(t1554: F, t373: F, t7205: F, t136559: F, t8042: F, t136565: F, t92470: F, t1293: F, t22794: F, t39: F, t92353: F, t136825: F, t32169: F, t32170: F, t136635: F, t64: F) -> (F, F, F, F, F, F) {
    let t136922 = t7205 * t1554 * t373;
    let t136926 = t8042 * t136559;
    let t136930 = t92470 * t136565;
    let t136935 = t92353 * t1293 * t39 * t22794;
    let t136952 = t32169 * t136825 * t32170;
    let t136967 = t64 * t136635;
    (t136922, t136926, t136930, t136935, t136952, t136967)
}
