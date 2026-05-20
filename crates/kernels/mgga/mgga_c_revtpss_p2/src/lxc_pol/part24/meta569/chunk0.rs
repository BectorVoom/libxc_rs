//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1744/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1744<F: Float>(t6628: F, t482: F, t1774: F, t24543: F, t1794: F, t24616: F, t17687: F, t5819: F, t1042: F, t1250: F, t1261: F, t12787: F, t13063: F, t17448: F, t17569: F, t21040: F, t24535: F, t24546: F, t247: F, t24759: F, t24787: F, t3618: F, t3625: F, t3720: F, t44375: F, t44378: F, t44448: F, t44449: F, t44609: F, t45371: F, t5391: F, t56731: F, t82749: F, t89837: F) -> (F, F, F, F, F, F) {
    let t90132 = t6628 * t6628;
    let t90133 = t482 * t90132;
    let t90162 = t1774 * t24543;
    let t90167 = t24616 * t1794;
    let t90180 = t17687 * t5819;
    let t90185 = F::cast_from(0.17149607247227894789e-2_f64) * t17569 * t24759 + F::cast_from(0.30011812682648815881e-2_f64) * t44448 * t1042 * t90133 * t44449 + F::cast_from(0.85748036236139473944e-3_f64) * t56731 * t24546 - F::cast_from(0.21437009059034868486e-3_f64) * t44375 * t1042 * t90133 * t44378 - F::cast_from(0.17149607247227894789e-2_f64) * t82749 - F::cast_from(0.85748036236139473944e-3_f64) * t45371 * t3720 * t90162 * t13063 - F::cast_from(0.51448821741683684368e-2_f64) * t44609 * t3720 * t90167 * t1250 + F::cast_from(0.71456696863449561621e-3_f64) * t1261 * t247 * t3618 * t89837 + F::cast_from(0.13550306960772657611e-1_f64) * t5391 * t24535 - F::cast_from(0.17149607247227894789e-2_f64) * t17448 * t24787 + F::cast_from(0.14291339372689912324e-2_f64) * t3625 * t12787 * t21040 * t90180;
    (t90132, t90133, t90162, t90167, t90180, t90185)
}
