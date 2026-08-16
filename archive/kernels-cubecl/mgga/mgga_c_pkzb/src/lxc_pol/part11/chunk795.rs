//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 795/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk795<F: Float>(t7930: F, t2203: F, t3046: F, t2215: F, t218: F, t3061: F, t675: F, t3065: F, t1174: F, t6149: F, t6165: F, t1171: F, t2196: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7957 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t7930;
    let t7966 = t2203 * t3046;
    let t7972 = t2215 * t3046;
    let t7979 = t218 * t675 * t3061;
    let t7980 = F::cast_from(0.32862666666666666666e0_f64) * t7979;
    let t7982 = t218 * t675 * t3065;
    let t7983 = F::cast_from(0.32862666666666666666e0_f64) * t7982;
    let t7996 = t6149 * t1174;
    let t7999 = t6165 * t1174;
    let t8009 = t1171 * t2196;
    (t7957, t7966, t7972, t7979, t7980, t7982, t7983, t7996, t7999, t8009)
}
