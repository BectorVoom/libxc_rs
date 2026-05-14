//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1108/1135 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1108<F: Float>(t33460: F, t33462: F, t33464: F, t33466: F, t33468: F, t33470: F, t33472: F, t33474: F, t33477: F, t33479: F, t33482: F, t33487: F, t33492: F, t33495: F, t33501: F, t33505: F, t33507: F, t33510: F, t33513: F, t33518: F, t33528: F, t33532: F) -> (F, F) {
    let t37773 = -0.67528199161846004231e-6 * t33460 - 0.90037598882461338975e-6 * t33462 - 0.3623181683912940217e-6 * t33464 + 0.14068374825384584215e-7 * t33466 - 0.30917817036057089854e-5 * t33468 + 0.5497187869010950576e-5 * t33470 + 0.45018799441230669488e-6 * t33472 - 0.11382560960801989336e-6 * t33474 - 0.18550690221634253912e-3 * t33477 + 0.18550690221634253912e-3 * t33479 + 0.15458908518028544927e-5 * t33482;
    let t37786 = -0.2748593934505475288e-5 * t33487 - 0.49163213094075520838e-7 * t33492 + 0.26681141802169376784e-7 * t33495 - 0.52388299421926781621e-9 * t33501 + 0.49163213094075520838e-8 * t33505 + 0.13526544953274976811e-4 * t33507 - 0.12670134934408760308e-4 * t33510 - 0.99041358770707472872e-5 * t33513 + 0.19336232562226912507e-7 * t33518 + 0.56397344973161828145e-8 * t33528 - 0.11584123368602295139e-4 * t33532;
    (t37773, t37786)
}
