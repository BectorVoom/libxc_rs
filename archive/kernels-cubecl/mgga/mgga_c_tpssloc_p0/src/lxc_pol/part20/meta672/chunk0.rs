//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2526/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2526<F: Float>(t50946: F, t50948: F, t50950: F, t50952: F, t50954: F, t50957: F, t50961: F, t50966: F, t50968: F, t50970: F, t50972: F, t43780: F, t43782: F, t43784: F, t43786: F, t43788: F, t43816: F, t44053: F, t50976: F, t50978: F, t50987: F, t50990: F, t50994: F) -> (F, F) {
    let t51200 = F::cast_from(0.71752e1_f64) * t50946 + F::cast_from(0.79724444444444444445e0_f64) * t50948 + F::cast_from(0.39862222222222222222e0_f64) * t50950 + F::cast_from(0.19931111111111111112e0_f64) * t50952 + F::cast_from(0.11958666666666666667e1_f64) * t50954 - F::cast_from(0.59793333333333333333e0_f64) * t50957 - F::cast_from(0.59793333333333333333e0_f64) * t50961 - F::cast_from(0.35876000000000000001e1_f64) * t50966 + F::cast_from(0.10954222222222222222e0_f64) * t50968 + F::cast_from(0.54771111111111111111e-1_f64) * t50970 + F::cast_from(0.32862666666666666667e0_f64) * t50972;
    let t51212 = t44053 - F::cast_from(0.85199506172839506175e-1_f64) * t50976 - F::cast_from(0.73028148148148148149e-1_f64) * t50978 + F::cast_from(0.39862222222222222224e0_f64) * t43780 + F::cast_from(0.79724444444444444447e0_f64) * t43782 + F::cast_from(0.39862222222222222222e0_f64) * t43784 - F::cast_from(0.59793333333333333333e0_f64) * t43786 - F::cast_from(0.99655555555555555557e-1_f64) * t43788 - F::cast_from(0.93011851851851851855e0_f64) * t43816 + F::cast_from(0.10954222222222222222e0_f64) * t50987 + F::cast_from(0.43816888888888888889e0_f64) * t50990 - F::cast_from(0.35876e1_f64) * t50994;
    (t51200, t51212)
}
