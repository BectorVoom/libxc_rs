//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1029/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1029<F: Float>(t26651: F, t783: F, t26392: F, t26400: F, t26410: F, t26520: F, t26556: F, t26558: F, t26561: F, t26633: F, t26634: F, t2771: F, t7660: F, t899: F, t9010: F) -> (F, F) {
    let t26652 = t783 * t26651;
    let t26653 = -t26556 * t899 + F::cast_from(4.0_f64) * t26561 * t2771 + F::cast_from(2.0_f64) * t26634 * t2771 + F::cast_from(4.0_f64) * t7660 * t9010 + t26392 - t26400 + t26410 + t26520 + t26558 - t26633 + t26652;
    (t26652, t26653)
}
