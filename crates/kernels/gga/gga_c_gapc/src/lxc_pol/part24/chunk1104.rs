//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1104/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1104<F: Float>(t33353: F, t33356: F, t33358: F, t33360: F, t33364: F, t33369: F, t33371: F, t33375: F, t33377: F, t33380: F, t33383: F, t33387: F, t33390: F, t33394: F, t33396: F, t33402: F, t33405: F, t33407: F, t33409: F, t33413: F, t33417: F, t33420: F) -> (F, F) {
    let t37735 = 0.18115908419564701085e-6 * t33353 + 0.21135226489492151266e-6 * t33356 + 0.4637672555408563478e-4 * t33358 - 0.4637672555408563478e-4 * t33360 + 0.14339270485772026911e-8 * t33364 + 0.18937162934584967536e-3 * t33369 + 0.43284943850479925794e-3 * t33371 - 0.13526544953274976811e-4 * t33375 - 0.10869545051738820651e-5 * t33377 - 0.3623181683912940217e-6 * t33380 - 0.67632724766374884054e-5 * t33383;
    let t37747 = -0.15458908518028544927e-5 * t33387 + 0.13505639832369200846e-5 * t33390 - 0.19907553780332177015e-6 * t33394 - 0.21102562238076876322e-7 * t33396 + 0.88394205998751600034e-8 * t33402 - 0.84410248952307505288e-7 * t33405 - 0.7246363367825880434e-6 * t33407 + 0.21102562238076876322e-7 * t33409 + 0.67460644627686456801e-8 * t33413 + 0.11066378711890822966e-7 * t33417 - 0.13259130899812740005e-6 * t33420;
    (t37735, t37747)
}
