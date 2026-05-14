//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1103/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1103<F: Float>(t33245: F, t33248: F, t33252: F, t33254: F, t33259: F, t33261: F, t33263: F, t33265: F, t33270: F, t33275: F, t33278: F, t33320: F, t33324: F, t33326: F, t33330: F, t33333: F, t33336: F, t33339: F, t33341: F, t33343: F, t33346: F, t33349: F) -> (F, F) {
    let t37697 = -0.5497187869010950576e-5 * t33245 + 0.12670134934408760308e-4 * t33248 + 0.12670134934408760308e-4 * t33252 - 0.13900948042322754167e-2 * t33254 - 0.34414249165852864587e-7 * t33259 + 0.19563586942029072472e-5 * t33261 + 0.43440462632258606772e-4 * t33263 + 0.86880925264517213544e-4 * t33265 + 0.1769305705790386642e-5 * t33270 + 0.2023566393031464771e-7 * t33275 + 0.50004799207799907351e-2 * t33278;
    let t37722 = 0.86880925264517213544e-4 * t33320 - 0.13900948042322754167e-2 * t33324 - 0.6487109086417285278e-2 * t33326 + 0.28678540971544053822e-8 * t33330 - 0.4637672555408563478e-4 * t33333 - 0.24581606547037760419e-8 * t33336 + 0.32775475396050347226e-8 * t33339 + 0.6487109086417285278e-2 * t33341 - 0.12264067424302645642e-2 * t33343 + 0.44933721382698730017e-6 * t33346 + 0.2318836277704281739e-4 * t33349;
    (t37697, t37722)
}
