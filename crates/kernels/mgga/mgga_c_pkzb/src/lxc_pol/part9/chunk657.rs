//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 657/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk657<F: Float>(t1133: F, t751: F, t1138: F, t2131: F, t290: F, t2956: F, t2969: F, t2971: F, t2977: F, t2980: F, t2981: F, t791: F, t794: F) -> (F, F) {
    let t2984 = t751 * t1133;
    let t2989 = F::cast_from(0.13170898365871023197e1_f64) * t2969 * t2971 + F::cast_from(0.65854491829355115987e0_f64) * t2131 * t1138 + F::cast_from(0.65854491829355115987e0_f64) * t791 * t2977 - F::cast_from(0.65854491829355115987e0_f64) * t2980 * t2981 + F::cast_from(0.65854491829355115987e0_f64) * t2984 * t794 + F::cast_from(0.65854491829355115987e0_f64) * t290 * t2956;
    (t2984, t2989)
}
