//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1198/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1198<F: Float>(t10135: F, t1220: F, t10131: F, t11383: F, t904: F, t10258: F, t10263: F, t23318: F, t23332: F, t23338: F, t23341: F, t28227: F, t28231: F, t28234: F, t28263: F, t28266: F) -> (F,) {
    let t32169 = t1220 * t10135;
    let t32171 = t1220 * t10131;
    let t32177 = t11383 * t904;
    let t32183 = t32169 / 36.0 - t32171 / 18.0 + 0.51448821741683684367e-2 * t28227 - 0.34299214494455789578e-2 * t28231 + 0.17149607247227894789e-2 * t28234 + t23318 - t23332 - 5.0 / 432.0 * t23338 - 77.0 / 486.0 * t32177 + t23341 - 0.17149607247227894789e-2 * t28263 + 0.17149607247227894789e-2 * t28266 + 0.82318114786693894988e-1 * t10258 * t10263;
    (t32183,)
}
