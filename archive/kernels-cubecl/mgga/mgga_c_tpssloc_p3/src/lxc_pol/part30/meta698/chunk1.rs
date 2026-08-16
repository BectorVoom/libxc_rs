//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2238/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2238<F: Float>(t87198: F, t98610: F, t98612: F, t98614: F, t98616: F, t98618: F, t98620: F, t98622: F, t98624: F, t98626: F, t98629: F, t98631: F, t98633: F, t98635: F, t98637: F, t98639: F, t98642: F) -> F {
    let t98644 = t98610 / F::cast_from(192.0_f64) + t98612 / F::cast_from(192.0_f64) + t98614 / F::cast_from(192.0_f64) + t98616 / F::cast_from(384.0_f64) - F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t98618 + t98620 / F::cast_from(256.0_f64) + t98622 / F::cast_from(768.0_f64) - t87198 - t98624 / F::cast_from(1536.0_f64) - t98626 / F::cast_from(256.0_f64) + t98629 / F::cast_from(384.0_f64) - t98631 / F::cast_from(192.0_f64) + t98633 / F::cast_from(384.0_f64) + t98635 / F::cast_from(384.0_f64) - t98637 / F::cast_from(768.0_f64) - t98639 / F::cast_from(1536.0_f64) - F::cast_from(0.16956557559538964158e-1_f64) * t98642;
    t98644
}
