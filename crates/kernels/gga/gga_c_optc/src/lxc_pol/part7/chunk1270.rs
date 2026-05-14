//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1270/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1270<F: Float>(t1219: F, t176: F, t9337: F, t27071: F, t490: F, t492: F, t496: F, t1186: F, t1220: F, t1221: F, t1223: F, t26256: F, t26291: F, t26302: F, t26341: F, t26849: F, t26855: F, t26857: F, t2911: F, t3274: F, t3284: F, t3980: F, t8417: F, t8422: F, t8426: F, t914: F, t9221: F) -> (F,) {
    let t28088 = t176 * t9337 * t1219;
    let t28109 = 40.0 / 81.0 * t490 * t492 * t27071 * t496;
    let t28114 = t1220 * t914 * t1221 * t26341 / 6.0 + 2.0 / 3.0 * t28088 * t1223 + 2.0 / 3.0 * t1220 * t914 * t3284 * t26291 - t1220 * t914 * t1221 * t26302 - 16.0 / 3.0 * t3274 * t8417 - 56.0 / 9.0 * t1220 * t914 * t8426 * t26256 + 4.0 * t3274 * t8422 - t28109 - t26855 + t26857 - 0.10337952573961372198e-1 * t3980 * t9221 * t2911 * t1186 - t26849;
    (t28114,)
}
