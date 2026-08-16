//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3482/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3482(t1063: f64, t19929: f64, t3172: f64, t19933: f64, t19676: f64, t3127: f64, t1042: f64, t11994: f64, t12021: f64, t15193: f64, t15817: f64, t15970: f64, t15975: f64, t16138: f64, t19688: f64, t19738: f64, t19741: f64, t19792: f64, t19800: f64, t3124: f64, t3188: f64, t4583: f64, t4801: f64, t4823: f64, t4869: f64, t6302: f64, t65433: f64) -> f64 {
    let t65507 = t1063 * t3172 * t19929;
    let t65510 = t1063 * t3172 * t19933;
    let t65527 = t3127 * t3172 * t19676;
    let t65533 = 0.85748036236139473944e-3_f64 * t15817 * t4869 - 0.57165357490759649296e-3_f64 * t11994 * t19792 - 0.57165357490759649296e-3_f64 * t3127 * t1042 * t16138 * t4583 + 0.11433071498151929859e-2_f64 * t65507 - 0.76220476654346199061e-3_f64 * t65510 - 0.28582678745379824648e-3_f64 * t3127 * t1042 * t4823 * t15193 + 0.47637797908966374414e-3_f64 * t3188 * t19688 + 0.21437009059034868486e-3_f64 * t12021 * t6302 - 0.57165357490759649296e-3_f64 * t1063 * t1042 * t4801 * t65433 + 0.42874018118069736972e-3_f64 * t3124 * t19800 - 0.19055119163586549765e-3_f64 * t65527 + 0.57165357490759649296e-3_f64 * t19738 * t15970 - 0.28582678745379824648e-3_f64 * t19741 * t15975;
    t65533
}
