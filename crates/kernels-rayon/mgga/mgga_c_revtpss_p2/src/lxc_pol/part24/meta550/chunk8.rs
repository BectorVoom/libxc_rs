//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1634/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1634(t1583: f64, t18268: f64, t198: f64, t23114: f64, t2393: f64, t39770: f64, t39773: f64, t39783: f64, t39786: f64, t39791: f64, t39795: f64, t4541: f64, t5966: f64, t87548: f64, t87641: f64, t87642: f64, t87643: f64, t87644: f64, t87649: f64, t892: f64) -> f64 {
    let t87942 = 24.0_f64 * t1583 * t198 * t23114 * t892 - 36.0_f64 * t18268 * t4541 * t5966 + 18.0_f64 * t198 * t2393 * t87548 + t39770 + t39773 - t39783 - t39786 - t39791 - t39795 - t87641 + t87642 - t87643 + t87644 + t87649;
    t87942
}
