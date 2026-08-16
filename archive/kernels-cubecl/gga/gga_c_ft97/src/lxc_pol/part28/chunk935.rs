//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 935/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk935<F: Float>(t136240: F, t32071: F, t23054: F, t32096: F, t32356: F, t376: F, t89: F, t1557: F, t7165: F, t17: F, t171: F, t397: F) -> (F, F, F, F, F) {
    let t136241 = t136240 * t32071;
    let t136243 = t23054 * t32096;
    let t136250 = t89 * t376 * t32356;
    let t136269 = t7165 * t1557;
    let t136275 = t397 * t171 * t17;
    (t136241, t136243, t136250, t136269, t136275)
}
