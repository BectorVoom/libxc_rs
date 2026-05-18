//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1297/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1297<F: Float>(t33320: F, t33324: F, t33326: F, t33330: F, t33333: F, t33336: F, t33339: F, t33341: F, t33343: F, t33346: F, t33349: F, t33353: F, t33356: F, t33358: F, t33360: F, t33364: F, t33369: F, t33371: F, t33375: F, t33377: F, t33380: F, t33383: F) -> (F, F) {
    let t37722 = F::new(0.86880925264517213544e-4) * t33320 - F::new(0.13900948042322754167e-2) * t33324 - F::new(0.6487109086417285278e-2) * t33326 + F::new(0.28678540971544053822e-8) * t33330 - F::new(0.4637672555408563478e-4) * t33333 - F::new(0.24581606547037760419e-8) * t33336 + F::new(0.32775475396050347226e-8) * t33339 + F::new(0.6487109086417285278e-2) * t33341 - F::new(0.12264067424302645642e-2) * t33343 + F::new(0.44933721382698730017e-6) * t33346 + F::new(0.2318836277704281739e-4) * t33349;
    let t37735 = F::new(0.18115908419564701085e-6) * t33353 + F::new(0.21135226489492151266e-6) * t33356 + F::new(0.4637672555408563478e-4) * t33358 - F::new(0.4637672555408563478e-4) * t33360 + F::new(0.14339270485772026911e-8) * t33364 + F::new(0.18937162934584967536e-3) * t33369 + F::new(0.43284943850479925794e-3) * t33371 - F::new(0.13526544953274976811e-4) * t33375 - F::new(0.10869545051738820651e-5) * t33377 - F::new(0.3623181683912940217e-6) * t33380 - F::new(0.67632724766374884054e-5) * t33383;
    (t37722, t37735)
}
