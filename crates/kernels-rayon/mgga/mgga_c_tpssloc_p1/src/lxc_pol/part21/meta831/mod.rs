//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta831 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2928;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2929;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2930;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2931;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta831(t1068: f64, t4696: f64, t13508: f64, t4483: f64, t17934: f64, t2948: f64, t13718: f64, t10723: f64, t17954: f64, t959: f64, t17937: f64, t2925: f64, t14667: f64, t17198: f64, t17202: f64, t3209: f64, t4700: f64, t60398: f64, t60400: f64, t60429: f64, t60434: f64, t60568: f64, t60570: f64, t17297: f64, t2929: f64, t4497: f64, t2904: f64, t59975: f64, t951: f64, t18065: f64, t225: f64, t10165: f64, t10170: f64, t1052: f64, t1066: f64, t11010: f64, t13939: f64, t14658: f64, t1625: f64, t1634: f64, t1635: f64, t17583: f64, t18062: f64, t18166: f64, t3026: f64, t3169: f64, t3174: f64, t3175: f64, t3206: f64, t388: f64, t43604: f64, t4552: f64, t4657: f64, t50625: f64, t50632: f64, t50653: f64, t50703: f64, t5919: f64, t5920: f64, t5944: f64, t10160: f64, t13736: f64, t13743: f64, t14526: f64, t14545: f64, t14549: f64, t14555: f64, t14659: f64, t1603: f64, t17575: f64, t17588: f64, t3166: f64, t3176: f64, t4557: f64, t4660: f64, t4665: f64, t4694: f64, t5848: f64, t5943: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t60941, t60946, t60953, t60955, t60958, t60961) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2928(t1068, t4696, t13508, t4483, t17934, t2948, t13718, t10723, t17954, t959, t17937, t2925);
        let t60962 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2929(t14667, t17198, t17202, t3209, t4700, t60398, t60400, t60429, t60434, t60568, t60570, t60941, t60946, t60953, t60955, t60958, t60961);
        let (t60966, t60970, t61010) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2930(t17297, t2929, t4497, t959, t2904, t59975, t951, t18065, t225, t10165, t10170, t1052, t1066, t11010, t13939, t14658, t1625, t1634, t1635, t17583, t18062, t18166, t3026, t3169, t3174, t3175, t3206, t388, t43604, t4552, t4657, t50625, t50632, t50653, t50703, t5919, t5920, t5944);
        let t61048 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2931(t10160, t1052, t13736, t13743, t14526, t14545, t14549, t14555, t14659, t1603, t17575, t17583, t17588, t18062, t3166, t3169, t3174, t3176, t3206, t388, t4557, t4660, t4665, t4694, t5848, t5943, t5944);
    (t60946, t60953, t60955, t60958, t60961, t60962, t60966, t60970, t61010, t61048)
}
