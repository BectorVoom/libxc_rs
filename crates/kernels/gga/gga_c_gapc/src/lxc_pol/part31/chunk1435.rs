//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1435/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1435<F: Float>(t33779: F, t36698: F, t36699: F, t36700: F, t36701: F, t36703: F, t36704: F, t36705: F, t36706: F, t36707: F, t36708: F, t33834: F, t33838: F, t36723: F, t36725: F, t36727: F, t36728: F, t36729: F, t36730: F, t36731: F, t36732: F, t36733: F) -> (F, F) {
    let t38763 = -t36698 - t36699 - t36700 + t36701 - F::new(0.57970906942607043475e-5) * t33779 - t36703 + t36704 + t36705 - t36706 + t36707 + t36708;
    let t38767 = -t36723 + F::new(0.2445773654513888889e-4) * t33834 - t36725 - F::new(0.18115908419564701086e-6) * t33838 + t36727 - t36728 + t36729 - t36730 + t36731 + t36732 - t36733;
    (t38763, t38767)
}
