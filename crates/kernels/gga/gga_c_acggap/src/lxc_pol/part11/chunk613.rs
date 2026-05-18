//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 613/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk613<F: Float>(t1465: F, t4680: F, t1579: F, t407: F, t1181: F, t1470: F, t3382: F, t1180: F, t3460: F, t3477: F, t3479: F, t3494: F, t3504: F, t3532: F, t3551: F, t3552: F, t3556: F, t3562: F, t3563: F, t418: F, t4667: F, t4670: F, t4673: F, t4675: F, t4677: F, t4679: F) -> (F, F, F) {
    let t4681 = t4680 * t1465;
    let t4684 = t1579 * t407;
    let t4685 = t1181 * t4684;
    let t4689 = F::new(0.85748036236139473944e-3) * t3382 * t1470;
    let t4690 = -F::new(0.42874018118069736972e-3) * t3460 - F::new(0.42874018118069736972e-3) * t3477 - F::new(0.21437009059034868486e-3) * t3479 - F::new(0.42874018118069736972e-2) * t3494 - F::new(7.0) / F::new(72.0) * t3504 + F::new(0.42874018118069736972e-3) * t3532 - t3551 + F::new(0.42874018118069736972e-3) * t3552 + t3556 - t3562 - F::new(0.20007875121765877254e-2) * t3563 + F::new(0.12862205435420921092e-2) * t418 * t4667 + F::new(0.40015750243531754508e-2) * t4670 + t4673 - t4675 + t4677 + t4679 - F::new(0.85748036236139473944e-3) * t1180 * t4681 - F::new(0.85748036236139473944e-3) * t1180 * t4685 + t4689;
    (t4681, t4685, t4690)
}
