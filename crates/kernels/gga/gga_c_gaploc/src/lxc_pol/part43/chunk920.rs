//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 920/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk920<F: Float>(t1382: F, t14479: F, t605: F, t42520: F, t44223: F, t44225: F, t44228: F, t44231: F, t44232: F, t44234: F, t44236: F, t44238: F, t44239: F, t44242: F, t47786: F, t48242: F, t50987: F, t51074: F, t51075: F, t51082: F, t51092: F, t51104: F, t51107: F, t51115: F, t51120: F, t51126: F, t51134: F, t51142: F, t51146: F, t51152: F, t51156: F, t51162: F, t51171: F, t51183: F, t51188: F, t748: F) -> (F, F) {
    let t51197 = 2.0 * t1382 * t14479 * t605;
    let t51198 = t44223 + t44225 + t50987 - t44228 + t42520 + t44231 + 4.0 * t47786 - t44232 - t44234 + t51074 + t44236 + t44238 - t44239 + t51075 - t748 * (t51082 + t51092 + t51104 + t51107 + t51115 + t51120 + t51126 + t51134 + t51142 + t51146 + t51152 + t51156 + t51162 + t51171 + t51183 + t51188) + t44242 - 2.0 * t48242 - t51197;
    (t51197, t51198)
}
