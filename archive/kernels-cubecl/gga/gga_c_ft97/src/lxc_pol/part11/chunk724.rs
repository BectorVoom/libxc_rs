//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 724/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk724<F: Float>(t2526: F, t766: F, t2568: F, t242: F, t1901: F, t446: F, t9788: F, t9794: F, t9799: F, t9805: F, t9810: F, t9813: F, t9816: F, t9819: F, t9822: F, t9824: F, t9826: F, t9828: F, t9831: F, t9835: F) -> (F, F, F, F) {
    let t9838 = t766 * t2526;
    let t9839 = t2568 * t9838;
    let t9840 = t242 * t9839;
    let t9843 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t9788 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t9794 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t9799 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t9805 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t9810 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t9813 - t446 * t9816 - t446 * t9819 / F::cast_from(3.0_f64) - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t9822 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t9824 + t9826 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t9828 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t9831 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t446 * t9835 + F::cast_from(2.0_f64) * t446 * t9840;
    (t9838, t9839, t9840, t9843)
}
