//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1085/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1085<F: Float>(t2104: F, t276: F, t2887: F, t2899: F, t2922: F, t3631: F, t5691: F, t735: F, t757: F, t7718: F, t7756: F, t7760: F, t7767: F, t9542: F, t9547: F, t9550: F, t9555: F, t9559: F, t9564: F, t9568: F, t9572: F, t9577: F, t9584: F) -> (F,) {
    let t9586 = -t7718 + t5691 / 432.0 + 0.19055119163586549765e-3 * t7756 + 0.30488190661738479625e-2 * t7760 - t7767 + 0.21437009059034868486e-3 * t757 * t9542 - t9547 / 288.0 - t276 * t9550 / 96.0 + t2887 * t9555 / 48.0 - 0.42874018118069736972e-3 * t2104 * t9559 + 0.42874018118069736972e-3 * t2899 * t9564 - 0.21437009059034868486e-3 * t2922 * t9568 - 0.85748036236139473944e-3 * t2104 * t9572 - 0.85748036236139473944e-3 * t2104 * t9577 - t735 * t3631 / 18.0 + t9584 / 144.0;
    (t9586,)
}
