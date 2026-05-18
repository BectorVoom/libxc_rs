//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 717/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk717<F: Float>(t596: F, t6838: F, t6480: F, t6484: F, t6488: F, t6492: F, t6816: F, t6819: F, t6823: F, t6827: F, t6829: F, t6832: F, t6834: F, t6836: F) -> (F, F) {
    let t6840 = F::new(0.56969282336565386482e-3) * t596 * t6838;
    let t6841 = t6816 - t6819 - t6480 - t6484 + t6488 - t6823 + t6827 + t6829 + t6832 + t6834 + t6836 + t6492 - t6840;
    (t6840, t6841)
}
