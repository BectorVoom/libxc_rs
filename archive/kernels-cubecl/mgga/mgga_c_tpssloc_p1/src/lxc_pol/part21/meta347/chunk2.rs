//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1747/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1747<F: Float>(t13025: F, t2576: F, t13005: F, t13007: F, t13010: F, t13014: F, t13017: F, t13020: F, t13022: F, t787: F, t9572: F, t9574: F, t9579: F, t9583: F) -> (F, F) {
    let t13027 = F::cast_from(0.16666666666666666666e-2_f64) * t2576 * t13025;
    let t13028 = -F::cast_from(0.19999999999999999999e-1_f64) * t13005 * t13007 - t9572 - F::cast_from(0.12962962962962962962e-1_f64) * t13010 - t13014 - F::cast_from(0.11666666666666666666e-1_f64) * t9574 + t9579 - F::cast_from(0.16666666666666666666e-2_f64) * t787 * t13017 + F::cast_from(0.77777777777777777774e-2_f64) * t13020 - F::cast_from(0.52777777777777777776e-2_f64) * t13022 + t13027 - t9583;
    (t13027, t13028)
}
