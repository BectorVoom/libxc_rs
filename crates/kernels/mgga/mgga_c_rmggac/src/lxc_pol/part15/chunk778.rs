//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 778/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk778<F: Float>(t1528: F, t1970: F, t209: F, t236: F, t605: F, t7231: F, t1494: F, t618: F, t10078: F, t7255: F, t1587: F, t3352: F, t39832: F, t8443: F, t41890: F, t39513: F, t8451: F) -> (F, F, F, F, F, F, F) {
    let t44676 = t1970 * t7231 * t236 * t1528 * t605 * t209;
    let t44682 = t1970 * t7231 * t236 * t618 * t1494 * t209;
    let t44684 = t7255 * t10078;
    let t44690 = t1970 * t3352 * t236 * t1587 * t605 * t209;
    let t44692 = t39832 * t8443;
    let t44694 = t41890 * t8443;
    let t44696 = t8451 * t39513;
    (t44676, t44682, t44684, t44690, t44692, t44694, t44696)
}
