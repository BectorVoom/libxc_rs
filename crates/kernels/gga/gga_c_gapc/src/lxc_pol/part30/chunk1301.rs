//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1301/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1301<F: Float>(t33387: F, t33390: F, t33394: F, t33396: F, t33402: F, t33405: F, t33407: F, t33409: F, t33413: F, t33417: F, t33420: F, t33460: F, t33462: F, t33464: F, t33466: F, t33468: F, t33470: F, t33472: F, t33474: F, t33477: F, t33479: F, t33482: F) -> (F, F) {
    let t37747 = -F::new(0.15458908518028544927e-5) * t33387 + F::new(0.13505639832369200846e-5) * t33390 - F::new(0.19907553780332177015e-6) * t33394 - F::new(0.21102562238076876322e-7) * t33396 + F::new(0.88394205998751600034e-8) * t33402 - F::new(0.84410248952307505288e-7) * t33405 - F::new(0.7246363367825880434e-6) * t33407 + F::new(0.21102562238076876322e-7) * t33409 + F::new(0.67460644627686456801e-8) * t33413 + F::new(0.11066378711890822966e-7) * t33417 - F::new(0.13259130899812740005e-6) * t33420;
    let t37773 = -F::new(0.67528199161846004231e-6) * t33460 - F::new(0.90037598882461338975e-6) * t33462 - F::new(0.3623181683912940217e-6) * t33464 + F::new(0.14068374825384584215e-7) * t33466 - F::new(0.30917817036057089854e-5) * t33468 + F::new(0.5497187869010950576e-5) * t33470 + F::new(0.45018799441230669488e-6) * t33472 - F::new(0.11382560960801989336e-6) * t33474 - F::new(0.18550690221634253912e-3) * t33477 + F::new(0.18550690221634253912e-3) * t33479 + F::new(0.15458908518028544927e-5) * t33482;
    (t37747, t37773)
}
