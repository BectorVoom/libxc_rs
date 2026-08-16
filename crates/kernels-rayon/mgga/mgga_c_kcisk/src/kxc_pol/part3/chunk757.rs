//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 757/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk757(t11676: f64, t11677: f64, t5192: f64, t5182: f64, t1849: f64, t642: f64, t11219: f64, t10504: f64, t10777: f64, t11201: f64, t11239: f64, t11241: f64, t11245: f64, t11453: f64, t11456: f64, t11650: f64, t11652: f64, t11661: f64, t11663: f64, t11669: f64, t11674: f64, t1693: f64, t4823: f64, t4827: f64, t4830: f64, t671: f64) -> (f64, f64, f64) {
    let t11678 = t11676 * t11677;
    let t11679 = t5192 * t11678;
    let t11680 = t5182 * t11679;
    let t11682 = t642 * t1849;
    let t11683 = t11682 * t11219;
    let t11684 = t5192 * t11683;
    let t11685 = t5182 * t11684;
    let t11687 = 0.1492375e-1_f64 * t11239 + 0.99491666666666666664e-2_f64 * t11241 - 0.386e0_f64 * t1693 * t11201 + 0.223494e0_f64 * t11245 * t4827 - 0.24872916666666666666e-2_f64 * t11453 + 0.49745833333333333332e-2_f64 * t11456 + 0.24872916666666666666e-2_f64 * t11650 + 0.49745833333333333332e-2_f64 * t11652 + 0.579e0_f64 * t4830 * t4827 - 0.223494e0_f64 * t4823 * t11201 - 0.74618749999999999998e-2_f64 * t11661 - 0.99491666666666666664e-2_f64 * t11663 + t10777 * t671 + 0.223494e0_f64 * t4823 * t10504 - 0.99491666666666666664e-2_f64 * t11669 + 0.1492375e-1_f64 * t11674 - 0.11054629629629629629e-2_f64 * t11680 - 0.66327777777777777775e-2_f64 * t11685;
    (t11680, t11685, t11687)
}
