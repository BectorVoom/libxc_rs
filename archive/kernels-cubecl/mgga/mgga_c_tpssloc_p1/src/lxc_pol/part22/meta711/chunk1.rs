//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2309/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2309<F: Float>(t40761: F, t16689: F, t4101: F, t16701: F, t4205: F, t20741: F, t706: F, t708: F, t20234: F, t751: F, t9897: F, t13133: F, t5597: F) -> (F, F, F, F, F, F) {
    let t67176 = F::cast_from(0.10254018858216406658e4_f64) * t40761;
    let t67177 = t16689 * t4101;
    let t67178 = F::cast_from(12.0_f64) * t67177;
    let t67179 = t4205 * t16701;
    let t67180 = F::cast_from(12.0_f64) * t67179;
    let t67181 = t706 * t20741;
    let t67183 = F::cast_from(4.0_f64) * t67181 * t708;
    let t67185 = t9897 * t751 * t20234;
    let t67186 = F::cast_from(24.0_f64) * t67185;
    let t67191 = F::cast_from(12.0_f64) * t13133 * t5597;
    (t67176, t67178, t67180, t67183, t67186, t67191)
}
