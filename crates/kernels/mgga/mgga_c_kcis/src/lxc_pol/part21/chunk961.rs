//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 961/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk961<F: Float>(t26651: F, t783: F, t26392: F, t26400: F, t26410: F, t26520: F, t26556: F, t26558: F, t26561: F, t26633: F, t26634: F, t2771: F, t7660: F, t899: F, t9010: F, t26519: F) -> (F, F) {
    let t26652 = t783 * t26651;
    let t26653 = -t26556 * t899 + 4.0 * t26561 * t2771 + 2.0 * t26634 * t2771 + 4.0 * t7660 * t9010 + t26392 - t26400 + t26410 + t26520 + t26558 - t26633 + t26652;
    let t26654 = t26519 + t26653;
    (t26652, t26654)
}
