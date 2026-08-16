//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2257/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2257<F: Float>(t29313: F, t3801: F, t12587: F, t8220: F, t104509: F, t104560: F, t104601: F, t105057: F, t105107: F, t105155: F, t105206: F, t105258: F, t105310: F, t105358: F, t105402: F, t105457: F, t105504: F, t105553: F, t105613: F, t105657: F, t1298: F, t1300: F, t18123: F, t1832: F, t198: F, t27037: F, t27041: F, t29317: F, t29322: F, t336: F, t3794: F, t3798: F, t5023: F, t5501: F, t73262: F, t7673: F, t97487: F, t97491: F, t97498: F) -> F {
    let t105665 = t29313 * t3801;
    let t105669 = t8220 * t12587;
    let t105696 = t198 * t336 * (t104509 + t104560 + t104601 + t105057 + t105107 + t105155 + t105206 + t105258 + t105310 + t105358 + t105402 + t105457 + t105504 + t105553 + t105613 + t105657) * t1300 - F::cast_from(2.0_f64) * t5023 * t105665 * t1298 + F::cast_from(2.0_f64) * t5023 * t105669 * t3798 - t5023 * t29317 * t3794 - t5023 * t97487 * t1832 + F::cast_from(4.0_f64) * t5023 * t97491 * t29322 - F::cast_from(2.0_f64) * t5023 * t27037 * t5501 - F::cast_from(6.0_f64) * t5023 * t97498 * t1832 * t3798 + F::cast_from(4.0_f64) * t5023 * t27041 * t73262 + F::cast_from(2.0_f64) * t5023 * t27041 * t1832 * t3794 - t5023 * t7673 * t18123;
    t105696
}
