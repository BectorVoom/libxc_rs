//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 875/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk875(t10066: f64, t36772: f64, t40759: f64, t8626: f64, t623: f64, t8629: f64, t8632: f64, t1734: f64, t352: f64, t3928: f64, t6418: f64, t645: f64) -> (f64, f64, f64, f64, f64) {
    let t44700 = t36772 * t10066;
    let t44702 = t40759 * t8626;
    let t44705 = t623 * t8629 * t8632;
    let t44713 = t1734 * t352;
    let t44724 = t3928 * t645 * t6418;
    (t44700, t44702, t44705, t44713, t44724)
}
