//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 853/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk853<F: Float>(t5522: F, t5543: F, t7357: F, t7451: F, t9138: F, t9140: F, t9143: F, t9148: F, t9163: F, t9165: F, t9172: F, t9174: F) -> F {
    let t9176 = F::new(0.19419375e1) * t9138 - F::new(0.258925e1) * t9140 - F::new(0.1294625e1) * t9143 + F::new(0.258925e1) * t9165 - t5543 + F::cast_from(0.40256666666666666667e0_f64) * t5522 + F::cast_from(0.80513333333333333333e0_f64) * t7357 - t7451 - F::new(0.301925e0) * t9148 + F::new(0.905775e0) * t9163 - F::cast_from(0.412621875e-1_f64) * t9172 + F::new(0.16504875e0) * t9174;
    t9176
}
