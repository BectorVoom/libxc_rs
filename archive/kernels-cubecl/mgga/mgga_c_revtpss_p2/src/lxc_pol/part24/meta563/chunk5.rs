//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1700/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1700<F: Float>(t6244: F, t6299: F, t1011: F, t1042: F, t1045: F, t11774: F, t11927: F, t15696: F, t15926: F, t19611: F, t23903: F, t23912: F, t23916: F, t23999: F, t3091: F, t3092: F, t3117: F, t3127: F, t42328: F, t43069: F, t4872: F, t4919: F, t55122: F, t5825: F, t6258: F, t6267: F, t66306: F, t79107: F, t79112: F, t79139: F, t79141: F, t79155: F, t88132: F, t88828: F) -> (F, F) {
    let t89035 = t6244 * t6299;
    let t89046 = F::cast_from(0.22866142996303859718e-2_f64) * t79107 + F::cast_from(0.57165357490759649296e-3_f64) * t79112 - F::cast_from(0.85748036236139473944e-3_f64) * t3127 * t1042 * t4872 * t5825 * t6258 + F::cast_from(0.34299214494455789578e-2_f64) * t43069 * t66306 * t6267 - F::cast_from(0.17149607247227894789e-2_f64) * t11774 * t15696 * t23912 - F::cast_from(0.28582678745379824648e-2_f64) * t11774 * t55122 * t23916 + F::cast_from(0.17149607247227894789e-2_f64) * t42328 * t15696 * t23903 - F::cast_from(0.22866142996303859718e-2_f64) * t79139 - F::cast_from(0.17149607247227894789e-2_f64) * t3091 * t3092 * t19611 * t88828 + F::cast_from(0.57165357490759649296e-3_f64) * t79141 + F::cast_from(0.25724410870841842184e-2_f64) * t11927 * t3117 * t89035 * t1045 - F::cast_from(0.34299214494455789578e-2_f64) * t79155 - F::cast_from(0.25724410870841842184e-2_f64) * t15926 * t23999 + t1011 * t4919 * t88132 / F::cast_from(54.0_f64);
    (t89035, t89046)
}
