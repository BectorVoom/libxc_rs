//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 343/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk343<F: Float>(t1169: F, t1175: F, t1178: F, t1182: F, t865: F, t868: F) -> (F,) {
    let t1196 = 0.3529725e1 * t1175 - t865 + 0.1549425e1 * t1169 + 0.6311625e0 * t1178 - t868 + 0.312585e0 * t1182;
    (t1196,)
}
