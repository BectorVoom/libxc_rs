//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 883/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk883(t17057: f64, t8673: f64, t1869: f64, t2527: f64, t8939: f64, t1801: f64, t5062: f64, t1693: f64, t23874: f64, t23876: f64, t23878: f64, t23880: f64, t23894: f64, t23922: f64, t2470: f64, t28262: f64, t28329: f64, t4823: f64, t7278: f64, t8846: f64, t8852: f64) -> (f64, f64, f64, f64) {
    let t28731 = t17057 * t8673;
    let t28732 = t1869 * t28731;
    let t28749 = t8939 * t2527;
    let t28750 = t1801 * t28749;
    let t28751 = t5062 * t28750;
    let t28752 = t1869 * t28751;
    let t28754 = 0.1492375e-1_f64 * t28732 - 0.386e0_f64 * t1693 * t28262 - 0.579e0_f64 * t23922 * t2470 + 0.33163888888888888887e-2_f64 * t23874 - 0.99491666666666666664e-2_f64 * t23876 - 0.11054629629629629629e-2_f64 * t23878 - 0.17687407407407407407e-1_f64 * t23880 - 0.579e0_f64 * t7278 * t8846 + 0.223494e0_f64 * t4823 * t28329 + 0.579e0_f64 * t7278 * t8852 - 0.49745833333333333332e-2_f64 * t23894 + 0.1492375e-1_f64 * t28752;
    (t28732, t28749, t28752, t28754)
}
