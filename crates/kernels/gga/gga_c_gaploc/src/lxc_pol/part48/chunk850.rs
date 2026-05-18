//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 850/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk850<F: Float>(t39624: F, t39626: F, t39632: F, t39646: F, t39648: F, t39650: F, t42114: F, t42115: F, t44590: F, t44592: F, t44593: F, t493: F) -> (F, F) {
    let t44595 = F::new(7.0) / F::new(256.0) * t39624;
    let t44596 = F::new(63.0) / F::new(8192.0) * t39626;
    let t44597 = F::new(63.0) / F::new(524288.0) * t39632;
    let t44598 = F::new(21.0) / F::new(524288.0) * t39646;
    let t44599 = F::new(21.0) / F::new(8192.0) * t39648;
    let t44600 = F::new(7.0) / F::new(768.0) * t39650;
    let t44601 = t44590 - t44592 + t44593 / F::new(2.0) + t42114 - t42115 + t44595 + t44596 - t44597 + t44598 - t44599 - t44600;
    let t44609 = t493 * t44601;
    (t44601, t44609)
}
