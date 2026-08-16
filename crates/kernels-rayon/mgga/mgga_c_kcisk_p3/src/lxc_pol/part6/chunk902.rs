//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 902/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk902(t2464: f64, t8510: f64, t10802: f64, t28377: f64, t7234: f64, t2448: f64, t8832: f64, t20: f64, t28957: f64, t649: f64, t10795: f64, t1773: f64, t23338: f64, t23769: f64, t2460: f64, t29011: f64, t29017: f64, t29025: f64, t29029: f64, t29032: f64, t5013: f64, t664: f64, t7208: f64, t8807: f64, t8811: f64) -> f64 {
    let t29035 = t8510 * t2464;
    let t29036 = t10802 * t29035;
    let t29039 = t7234 * t28377;
    let t29042 = t2448 * t8832;
    let t29045 = t28957 * t20;
    let t29046 = t649 * t29045;
    let t29049 = 0.53972366148531951639e-1_f64 * t7208 * t8807 + 0.17990788716177317213e-1_f64 * t1773 * t29011 + 0.71963154864709268852e-1_f64 * t7208 * t8811 + 0.55971342672551653552e-1_f64 * t1773 * t29017 + 0.52772980234120130492e0_f64 * t23769 * t2460 - 0.28785261945883707541e0_f64 * t23338 * t2460 + 0.32383419689119170984e0_f64 * t1773 * t29025 - 0.53972366148531951639e-1_f64 * t5013 * t29029 - 0.10794473229706390328e0_f64 * t5013 * t29032 - 0.71963154864709268852e-1_f64 * t5013 * t29036 + 0.71963154864709268853e-1_f64 * t5013 * t29039 + t10795 + 0.15831894070236039148e1_f64 * t29042 * t664 - 0.24627390775922727564e1_f64 * t29046 * t664;
    t29049
}
