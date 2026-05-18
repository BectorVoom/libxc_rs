//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1000/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1000<F: Float>(t1882: F, t9150: F, t9268: F, t9104: F, t605: F, t9132: F, t9442: F, t12703: F, t144: F, t1901: F, t3439: F, t379: F, t39668: F, t40522: F, t40760: F, t40766: F, t40771: F, t40772: F, t40777: F, t446: F, t558: F, t574: F, t9304: F, t9462: F) -> F {
    let t40779 = t1882 * t9150;
    let t40784 = t1882 * t9268;
    let t40786 = t1882 * t9104;
    let t40792 = t9132 * t605;
    let t40800 = t1882 * t9442;
    let t40802 = -F::new(8.0) / F::new(9.0) * t1901 * t3439 * t40766 * t40760 - F::new(16.0) / F::new(9.0) * t1901 * t3439 * t40771 * t40772 + F::new(16.0) / F::new(27.0) * t40777 + F::new(4.0) / F::new(3.0) * t40779 - F::new(4.0) / F::new(3.0) * t446 * t144 * t40522 + F::new(4.0) / F::new(3.0) * t40784 + F::new(4.0) / F::new(3.0) * t40786 - F::new(4.0) / F::new(3.0) * t446 * t574 * t9462 * t558 + F::new(8.0) / F::new(3.0) * t1901 * t40792 * t9304 * t379 - F::new(8.0) / F::new(3.0) * t1901 * t12703 * t39668 + F::new(8.0) / F::new(3.0) * t40800;
    t40802
}
