//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1091/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1091<F: Float>(t42724: F, t42740: F, t42757: F, t42772: F, t762: F, t10153: F, t10157: F, t1901: F, t242: F, t2579: F, t258: F, t2599: F, t2606: F, t3892: F, t42332: F, t42404: F, t42690: F, t42697: F, t42703: F, t42708: F, t446: F, t684: F, t729: F, t773: F, t9692: F, t9708: F) -> (F, F) {
    let t42775 = t762 * (t42724 + t42740 + t42757 + t42772);
    let t42783 = F::cast_from(4.0_f64) * t446 * t729 * t10153 * t2579 - F::cast_from(4.0_f64) * t1901 * t2606 * t3892 * t42404 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t42690 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t2599 * t258 * t9692 * t684 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t42697 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t729 * t773 * t9692 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t42703 + F::cast_from(2.0_f64) * t446 * t242 * t42332 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t42708 - t446 * t242 * t42775 / F::cast_from(3.0_f64) - F::cast_from(8.0_f64) * t446 * t10157 * t773 * t9708;
    (t42775, t42783)
}
