//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3170/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3170<F: Float>(t58359: F, t58372: F, t58386: F, t58399: F, t58413: F, t58426: F, t58440: F, t58453: F, t1130: F, t16807: F, t1151: F, t16835: F, t3428: F) -> (F, F, F) {
    let t58456 = t58359 + t58372 + t58386 + t58399 + t58413 + t58426 + t58440 + t58453;
    let t58460 = t16807 * t1130;
    let t58462 = F::new(3.0) * t58460 * t1151;
    let t58464 = F::new(3.0) * t16835 * t3428;
    (t58456, t58462, t58464)
}
