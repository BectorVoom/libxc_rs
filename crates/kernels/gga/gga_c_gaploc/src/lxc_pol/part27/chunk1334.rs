//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1334/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1334<F: Float>(t199: F, t20157: F, t31764: F, t196: F, t31770: F, t595: F, t10513: F, t580: F, t587: F, t20592: F, t2487: F, t18676: F, t34459: F, t6711: F) -> (F, F, F, F, F) {
    let t34733 = F::new(0.40899121017515466321e1) * t199 * t20157 * t31764;
    let t34737 = F::new(0.19427082483319846503e2) * t196 * t595 * t20157 * t31770;
    let t34740 = F::new(0.24539472610509279794e2) * t587 * t580 * t10513;
    let t34743 = F::new(0.11656249489991907902e3) * t2487 * t20592 * t10513;
    let t34746 = F::new(0.23005755572352449806e2) * t18676 * t6711 * t34459;
    (t34733, t34737, t34740, t34743, t34746)
}
