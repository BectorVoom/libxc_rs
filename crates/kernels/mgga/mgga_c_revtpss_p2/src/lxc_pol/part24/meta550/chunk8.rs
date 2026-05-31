//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1634/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1634<F: Float>(t1583: F, t18268: F, t198: F, t23114: F, t2393: F, t39770: F, t39773: F, t39783: F, t39786: F, t39791: F, t39795: F, t4541: F, t5966: F, t87548: F, t87641: F, t87642: F, t87643: F, t87644: F, t87649: F, t892: F) -> F {
    let t87942 = F::cast_from(24.0_f64) * t1583 * t198 * t23114 * t892 - F::cast_from(36.0_f64) * t18268 * t4541 * t5966 + F::cast_from(18.0_f64) * t198 * t2393 * t87548 + t39770 + t39773 - t39783 - t39786 - t39791 - t39795 - t87641 + t87642 - t87643 + t87644 + t87649;
    t87942
}
