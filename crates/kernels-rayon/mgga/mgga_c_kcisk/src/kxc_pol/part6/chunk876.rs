//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 876/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk876(t1835: f64, t28377: f64, t28385: f64, t706: f64, t11495: f64, t11630: f64, t158: f64, t173: f64, t1809: f64, t1850: f64, t23225: f64, t23229: f64, t23231: f64, t23234: f64, t23236: f64, t28312: f64, t28368: f64) -> f64 {
    let t28621 = t1835 * t28377;
    let t28624 = t1835 * t28385;
    let t28627 = t706 * t28377;
    let t28642 = -0.93231700340333523768e-3_f64 * t23225 - 0.2016525e-4_f64 * t173 * t28621 + 0.21078e-1_f64 * t158 * t28624 + 0.3513e-2_f64 * t158 * t28627 - 0.5179538907796306876e-4_f64 * t1850 * t28312 + 0.11955719325063177623e-1_f64 * t1809 * t28312 - 0.62154466893555682512e-3_f64 * t11630 * t28368 + 0.71734315950379065738e-1_f64 * t11495 * t28368 + 0.26416666666666666666e-2_f64 * t23229 - 0.352891875e-4_f64 * t23231 + 0.4705225e-4_f64 * t23234 + 0.70578375e-4_f64 * t23236;
    t28642
}
