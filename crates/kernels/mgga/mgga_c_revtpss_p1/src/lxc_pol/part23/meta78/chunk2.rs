//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 541/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk541<F: Float>(t1594: F, t1601: F, t1604: F, t1607: F, t948: F, t951: F) -> F {
    let t1621 = F::cast_from(0.3529725e1_f64) * t1601 - t948 - F::cast_from(0.516475e0_f64) * t1594 + F::cast_from(0.6311625e0_f64) * t1604 - t951 - F::cast_from(0.104195e0_f64) * t1607;
    t1621
}
