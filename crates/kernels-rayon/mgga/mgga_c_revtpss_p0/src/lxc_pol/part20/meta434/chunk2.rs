//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1637/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1637(t44864: f64, t44877: f64, t12984: f64, t3667: f64, t12976: f64, t3678: f64, t12963: f64, t1235: f64, t127: f64, t12970: f64, t371: f64, t1222: f64, t1238: f64, t12972: f64, t17693: f64, t17799: f64, t3663: f64, t372: f64, t43843: f64, t44800: f64, t44823: f64, t44829: f64, t44833: f64, t44838: f64, t44844: f64, t44845: f64, t482: f64, t5308: f64) -> (f64, f64) {
    let t44878 = t44864 + t44877;
    let t44884 = t3667 * t12984;
    let t44886 = t12976 * t3678;
    let t44888 = t3667 * t12963;
    let t44892 = t1235 * t371 * t127 * t12970;
    let t44894 = -0.34299214494455789577e-2_f64 * t17693 * t17799 * t44800 - t1222 * t5308 * t43843 / 8.0_f64 + 0.28582678745379824648e-3_f64 * t44823 - 0.12862205435420921092e-2_f64 * t12976 * t3663 - 0.2540682555144873302e-3_f64 * t44829 - 0.85748036236139473944e-3_f64 * t44833 * t1238 - 0.57165357490759649296e-3_f64 * t44838 + 0.51448821741683684368e-2_f64 * t44844 * t371 * t372 * t482 * t44845 - 0.85748036236139473944e-3_f64 * t3667 * t12972 - 0.21437009059034868486e-3_f64 * t1235 * t371 * t372 * t482 * t44878 + 0.57165357490759649296e-3_f64 * t44884 - 0.17149607247227894789e-2_f64 * t44886 - 0.17149607247227894789e-2_f64 * t44888 - 0.57165357490759649296e-3_f64 * t44892;
    (t44878, t44894)
}
