//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1043/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1043<F: Float>(t11449: F, t11804: F, t15843: F, t190: F, t2674: F, t11522: F, t15805: F, t9799: F, t3327: F, t33655: F, t33685: F, t7073: F, t3751: F, t9635: F, t11954: F, t3392: F) -> (F, F, F, F, F) {
    let t34132 = t2674 * t190 * t11449 * t11804 * t15843;
    let t34135 = t15805 * t11522 * t9799;
    let t34142 = t7073 * t33655 * t3327 * t33685;
    let t34144 = t3751 * t9635;
    let t34146 = t11954 * t3392;
    (t34132, t34135, t34142, t34144, t34146)
}
