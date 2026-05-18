//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1404/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1404<F: Float>(t1247: F, t1261: F, t12774: F, t12866: F, t12907: F, t12918: F, t12942: F, t12949: F, t12960: F, t17199: F, t17204: F, t17211: F, t17214: F, t17219: F, t17222: F, t17227: F, t17268: F, t17299: F, t17358: F, t17399: F, t17432: F, t17470: F, t17493: F, t17502: F, t17505: F, t17509: F, t17515: F, t17561: F, t17587: F, t17614: F, t17665: F, t17718: F, t17772: F, t17803: F, t3591: F, t3647: F, t3701: F, t3711: F, t3714: F, t5270: F, t5274: F, t5373: F, t5384: F) -> F {
    let t17807 = F::new(0.14291339372689912324e-3) * t12942 + t17772 + t17803 + F::new(0.21437009059034868486e-3) * t1247 * t17222 - F::new(0.57165357490759649296e-3) * t3647 * t5270 - F::new(0.28582678745379824648e-3) * t1261 * t17199 - F::new(0.85748036236139473944e-3) * t1261 * t17204 + F::new(0.21437009059034868486e-3) * t5274 * t3591 - F::new(0.28582678745379824648e-3) * t5384 * t17214 + F::new(0.28582678745379824648e-3) * t3711 * t17502 - F::new(0.15244095330869239812e-2) * t17505 * t3714 + F::new(0.28582678745379824648e-3) * t12866 * t17515 - t5373 * t3701 / F::new(81.0) - t17509 - t17227 + t17268 + F::new(0.28582678745379824648e-3) * t12907 + t17493 + t17614 + t17561 - F::new(0.19055119163586549765e-3) * t12774 + t17587 + t17718 + t17299 - F::new(0.14291339372689912324e-3) * t12949 + t17358 + F::new(0.19055119163586549765e-3) * t12960 + t17665 + t17219 + t17470 + t17432 - F::new(0.28582678745379824648e-3) * t12918 + t17211 + t17399;
    t17807
}
