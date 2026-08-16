//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1007/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1007<F: Float>(t78207: F, t8622: F, t5148: F, t570: F, t71949: F, t76435: F, t76440: F, t333: F, t5266: F, t77970: F, t14444: F, t1624: F) -> (F, F, F, F, F, F) {
    let t78208 = t78207 * t8622;
    let t78209 = F::cast_from(0.20455996240684006297e-1_f64) * t78208;
    let t78213 = t5148 * t71949 * t570;
    let t78214 = F::cast_from(0.79828278012425390427e-1_f64) * t78213;
    let t78215 = F::cast_from(0.79828278012425390427e-1_f64) * t76435;
    let t78216 = F::cast_from(0.14967802127329760705e-1_f64) * t76440;
    let t78219 = F::cast_from(0.11974241701863808564e0_f64) * t5266 * t77970 * t333;
    let t78220 = t14444 * t1624;
    (t78209, t78214, t78215, t78216, t78219, t78220)
}
