//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 648/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk648<F: Float>(t301: F, t3650: F, t761: F, t758: F, t2039: F, t3639: F, t179: F, t2004: F, t3542: F, t3515: F, t780: F, t2026: F, t2038: F, t2047: F, t2067: F, t2104: F, t276: F, t2884: F, t2909: F, t2940: F, t299: F, t3631: F, t3635: F, t3641: F, t3647: F, t757: F) -> (F, F, F, F, F, F, F, F) {
    let t3651 = t301 * t3650;
    let t3652 = t3651 * t761;
    let t3653 = t758 * t3652;
    let t3656 = t3639 * t2039;
    let t3657 = t758 * t3656;
    let t3662 = t179 * t2004 * t3542;
    let t3666 = t179 * t780 * t3515;
    let t3669 = -t2047 - t2884 / F::new(144.0) + t276 * t3631 / F::new(48.0) - t276 * t3635 / F::new(96.0) + F::new(0.42874018118069736972e-3) * t2026 * t3641 + F::new(0.28582678745379824648e-3) * t2909 - F::new(0.85748036236139473944e-3) * t2104 * t3647 + F::new(0.21437009059034868486e-3) * t757 * t3653 - F::new(0.21437009059034868486e-3) * t2038 * t3657 - t2067 - F::new(0.57165357490759649296e-3) * t2940 + F::new(0.12862205435420921092e-2) * t299 * t3662 - F::new(0.42874018118069736972e-3) * t299 * t3666;
    (t3651, t3652, t3653, t3656, t3657, t3662, t3666, t3669)
}
