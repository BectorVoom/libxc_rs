//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1744/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1744(t6628: f64, t482: f64, t1774: f64, t24543: f64, t1794: f64, t24616: f64, t17687: f64, t5819: f64, t1042: f64, t1250: f64, t1261: f64, t12787: f64, t13063: f64, t17448: f64, t17569: f64, t21040: f64, t24535: f64, t24546: f64, t247: f64, t24759: f64, t24787: f64, t3618: f64, t3625: f64, t3720: f64, t44375: f64, t44378: f64, t44448: f64, t44449: f64, t44609: f64, t45371: f64, t5391: f64, t56731: f64, t82749: f64, t89837: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t90132 = t6628 * t6628;
    let t90133 = t482 * t90132;
    let t90162 = t1774 * t24543;
    let t90167 = t24616 * t1794;
    let t90180 = t17687 * t5819;
    let t90185 = 0.17149607247227894789e-2_f64 * t17569 * t24759 + 0.30011812682648815881e-2_f64 * t44448 * t1042 * t90133 * t44449 + 0.85748036236139473944e-3_f64 * t56731 * t24546 - 0.21437009059034868486e-3_f64 * t44375 * t1042 * t90133 * t44378 - 0.17149607247227894789e-2_f64 * t82749 - 0.85748036236139473944e-3_f64 * t45371 * t3720 * t90162 * t13063 - 0.51448821741683684368e-2_f64 * t44609 * t3720 * t90167 * t1250 + 0.71456696863449561621e-3_f64 * t1261 * t247 * t3618 * t89837 + 0.13550306960772657611e-1_f64 * t5391 * t24535 - 0.17149607247227894789e-2_f64 * t17448 * t24787 + 0.14291339372689912324e-2_f64 * t3625 * t12787 * t21040 * t90180;
    (t90132, t90133, t90162, t90167, t90180, t90185)
}
