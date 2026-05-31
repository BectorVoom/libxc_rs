//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 495/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk495<F: Float>(t1882: F, t877: F, t2652: F, t2655: F, t2658: F, t2663: F, t2668: F, t2673: F, t2677: F, t2685: F, t2742: F, t2758: F, t2791: F) -> (F, F) {
    let t2819 = t1882 * t877;
    let t2823 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t2652;
    let t2832 = -t2758 / F::cast_from(12.0_f64) + t2791 / F::cast_from(6.0_f64) + t2823 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t2655 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t2658 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t2663 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t2668 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t2673 - t2677 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t2685 - t2742 / F::cast_from(3.0_f64);
    (t2819, t2832)
}
