//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1637/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1637<F: Float>(t44864: F, t44877: F, t12984: F, t3667: F, t12976: F, t3678: F, t12963: F, t1235: F, t127: F, t12970: F, t371: F, t1222: F, t1238: F, t12972: F, t17693: F, t17799: F, t3663: F, t372: F, t43843: F, t44800: F, t44823: F, t44829: F, t44833: F, t44838: F, t44844: F, t44845: F, t482: F, t5308: F) -> (F, F) {
    let t44878 = t44864 + t44877;
    let t44884 = t3667 * t12984;
    let t44886 = t12976 * t3678;
    let t44888 = t3667 * t12963;
    let t44892 = t1235 * t371 * t127 * t12970;
    let t44894 = -F::cast_from(0.34299214494455789577e-2_f64) * t17693 * t17799 * t44800 - t1222 * t5308 * t43843 / F::new(8.0) + F::cast_from(0.28582678745379824648e-3_f64) * t44823 - F::cast_from(0.12862205435420921092e-2_f64) * t12976 * t3663 - F::cast_from(0.2540682555144873302e-3_f64) * t44829 - F::cast_from(0.85748036236139473944e-3_f64) * t44833 * t1238 - F::cast_from(0.57165357490759649296e-3_f64) * t44838 + F::cast_from(0.51448821741683684368e-2_f64) * t44844 * t371 * t372 * t482 * t44845 - F::cast_from(0.85748036236139473944e-3_f64) * t3667 * t12972 - F::cast_from(0.21437009059034868486e-3_f64) * t1235 * t371 * t372 * t482 * t44878 + F::cast_from(0.57165357490759649296e-3_f64) * t44884 - F::cast_from(0.17149607247227894789e-2_f64) * t44886 - F::cast_from(0.17149607247227894789e-2_f64) * t44888 - F::cast_from(0.57165357490759649296e-3_f64) * t44892;
    (t44878, t44894)
}
