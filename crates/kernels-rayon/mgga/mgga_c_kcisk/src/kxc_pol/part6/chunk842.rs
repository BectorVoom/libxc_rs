//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 842/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk842(t1799: f64, t28300: f64, t2527: f64, t8780: f64, t5203: f64, t1873: f64, t1869: f64, t6719: f64, t8882: f64, t10447: f64, t967: f64) -> (f64, f64, f64, f64, f64) {
    let t28301 = t1799 * t28300;
    let t28303 = t8780 * t2527;
    let t28304 = t5203 * t28303;
    let t28305 = t1873 * t28304;
    let t28306 = t1869 * t28305;
    let t28308 = t6719 * t8882;
    let t28309 = t1869 * t28308;
    let t28312 = 6.0_f64 * t967 + 6.0_f64 * t10447;
    (t28301, t28303, t28306, t28309, t28312)
}
