//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1700/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1700(t6244: f64, t6299: f64, t1011: f64, t1042: f64, t1045: f64, t11774: f64, t11927: f64, t15696: f64, t15926: f64, t19611: f64, t23903: f64, t23912: f64, t23916: f64, t23999: f64, t3091: f64, t3092: f64, t3117: f64, t3127: f64, t42328: f64, t43069: f64, t4872: f64, t4919: f64, t55122: f64, t5825: f64, t6258: f64, t6267: f64, t66306: f64, t79107: f64, t79112: f64, t79139: f64, t79141: f64, t79155: f64, t88132: f64, t88828: f64) -> (f64, f64) {
    let t89035 = t6244 * t6299;
    let t89046 = 0.22866142996303859718e-2_f64 * t79107 + 0.57165357490759649296e-3_f64 * t79112 - 0.85748036236139473944e-3_f64 * t3127 * t1042 * t4872 * t5825 * t6258 + 0.34299214494455789578e-2_f64 * t43069 * t66306 * t6267 - 0.17149607247227894789e-2_f64 * t11774 * t15696 * t23912 - 0.28582678745379824648e-2_f64 * t11774 * t55122 * t23916 + 0.17149607247227894789e-2_f64 * t42328 * t15696 * t23903 - 0.22866142996303859718e-2_f64 * t79139 - 0.17149607247227894789e-2_f64 * t3091 * t3092 * t19611 * t88828 + 0.57165357490759649296e-3_f64 * t79141 + 0.25724410870841842184e-2_f64 * t11927 * t3117 * t89035 * t1045 - 0.34299214494455789578e-2_f64 * t79155 - 0.25724410870841842184e-2_f64 * t15926 * t23999 + t1011 * t4919 * t88132 / 54.0_f64;
    (t89035, t89046)
}
