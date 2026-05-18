//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1095/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1095<F: Float>(t2614: F, t8232: F, t10137: F, t1882: F, t8392: F, t9855: F, t10155: F, t10079: F, t10121: F, t1901: F, t1934: F, t2373: F, t2594: F, t2600: F, t2606: F, t265: F, t41691: F, t41718: F, t41726: F, t42884: F, t42894: F, t446: F, t684: F, t724: F, t761: F) -> F {
    let t42896 = t8232 * t2614;
    let t42898 = t1882 * t10137;
    let t42914 = t8392 * t9855;
    let t42916 = t1882 * t10155;
    let t42918 = -F::new(8.0) / F::new(27.0) * t42884 + F::new(8.0) / F::new(3.0) * t446 * t724 * t265 * t41691 - F::new(8.0) / F::new(3.0) * t446 * t2594 * t265 * t41718 + F::new(112.0) / F::new(243.0) * t42894 + F::new(16.0) / F::new(27.0) * t42896 + F::new(4.0) / F::new(9.0) * t42898 + F::new(4.0) / F::new(9.0) * t1901 * t2606 * t761 * t10121 * t684 - F::new(2.0) / F::new(9.0) * t446 * t2594 * t265 * t41726 - F::new(4.0) / F::new(3.0) * t1901 * t10079 * t2600 * t1934 * t2373 - F::new(8.0) / F::new(9.0) * t42914 + F::new(4.0) / F::new(3.0) * t42916;
    t42918
}
