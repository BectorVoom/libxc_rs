//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 425/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk425<F: Float>(t1852: F, t6547: F, t83: F, t5737: F, t5740: F, t6498: F, t6502: F, t6506: F, t6510: F, t6514: F, t6518: F, t6522: F) -> (F, F) {
    let t6548 = t1852 * t6547;
    let t6549 = t83 * t6548;
    let t6557 = t6498 / F::new(4.0) + t5737 + t6502 / F::new(6.0) + t6506 - t6510 / F::new(2.0) + t5740 + t6514 / F::new(3.0) + F::new(2.0) * t6518 - t6522;
    (t6549, t6557)
}
