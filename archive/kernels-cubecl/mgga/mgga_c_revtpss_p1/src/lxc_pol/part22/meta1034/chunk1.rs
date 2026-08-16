//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3619/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3619<F: Float>(t2439: F, t6464: F, t1145: F, t141: F, t68251: F, t6461: F, t3417: F, t68395: F, t58209: F, t58211: F, t58225: F, t68456: F, t68459: F, t68567: F, t68570: F, t68573: F, t68576: F, t68578: F, t68583: F) -> (F, F, F, F, F) {
    let t68585 = t2439 * t6464;
    let t68588 = t141 * t1145 * t68251;
    let t68590 = t2439 * t6461;
    let t68593 = t141 * t3417 * t68395;
    let t68595 = -F::cast_from(0.12077e1_f64) * t68456 + F::cast_from(0.181155e1_f64) * t68459 - F::cast_from(0.11038e0_f64) * t68567 + F::cast_from(0.82785e-1_f64) * t68570 - F::cast_from(0.5519e-1_f64) * t68573 - F::cast_from(0.27595e-1_f64) * t68576 + F::cast_from(0.16504875e0_f64) * t68578 - F::cast_from(0.22076e0_f64) * t58209 - F::cast_from(0.66228e0_f64) * t58211 + F::cast_from(0.73586666666666666667e0_f64) * t58225 + F::cast_from(0.91983333333333333334e-1_f64) * t68583 + F::cast_from(0.18396666666666666667e0_f64) * t68585 + F::cast_from(0.33114e0_f64) * t68588 - F::cast_from(0.30661111111111111112e-1_f64) * t68590 - F::cast_from(0.5519e-1_f64) * t68593;
    (t68585, t68588, t68590, t68593, t68595)
}
