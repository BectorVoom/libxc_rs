//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 1069/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk1069(t1382: f64, t14479: f64, t605: f64, t42520: f64, t44223: f64, t44225: f64, t44228: f64, t44231: f64, t44232: f64, t44234: f64, t44236: f64, t44238: f64, t44239: f64, t44242: f64, t47786: f64, t48242: f64, t50987: f64, t51074: f64, t51075: f64, t51082: f64, t51092: f64, t51104: f64, t51107: f64, t51115: f64, t51120: f64, t51126: f64, t51134: f64, t51142: f64, t51146: f64, t51152: f64, t51156: f64, t51162: f64, t51171: f64, t51183: f64, t51188: f64, t748: f64) -> (f64, f64) {
    let t51197 = 2.0_f64 * t1382 * t14479 * t605;
    let t51198 = t44223 + t44225 + t50987 - t44228 + t42520 + t44231 + 4.0_f64 * t47786 - t44232 - t44234 + t51074 + t44236 + t44238 - t44239 + t51075 - t748 * (t51082 + t51092 + t51104 + t51107 + t51115 + t51120 + t51126 + t51134 + t51142 + t51146 + t51152 + t51156 + t51162 + t51171 + t51183 + t51188) + t44242 - 2.0_f64 * t48242 - t51197;
    (t51197, t51198)
}
