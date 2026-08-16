//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1279/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1279<F: Float>(t1445: F, t32223: F, t833: F, t32219: F, t3431: F, t5750: F, t2615: F, t32514: F, t326: F, t14667: F, t2365: F, t25289: F) -> (F, F, F, F, F) {
    let t33030 = F::cast_from(0.11502877786176224903e2_f64) * t833 * t1445 * t32223;
    let t33033 = F::cast_from(0.23005755572352449806e2_f64) * t833 * t1445 * t32219;
    let t33034 = t5750 * t3431;
    let t33041 = F::cast_from(0.18404604457881959845e2_f64) * t2615 * t326 * t32514;
    let t33047 = t14667 * t2365 * t25289;
    (t33030, t33033, t33034, t33041, t33047)
}
