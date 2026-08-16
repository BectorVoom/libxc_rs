//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1233/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1233(t2385: f64, t943: f64, t32041: f64, t36019: f64, t8306: f64, t32181: f64, t36475: f64, t38086: f64, t310: f64, t464: f64, t9369: f64, t1659: f64, t2146: f64, t2222: f64, t33144: f64, t33147: f64, t33150: f64, t33153: f64, t33155: f64, t33157: f64, t33170: f64, t33173: f64, t5340: f64, t7890: f64, t8316: f64, t944: f64) -> (f64, f64) {
    let t38209 = t2385 * t943;
    let t38215 = t32041 * t8306 * t36019;
    let t38224 = t32181 * t38086 * t36475;
    let t38226 = t310 * t2385;
    let t38228 = 0.13170898365871023197e1_f64 * t38226 * t464;
    let t38232 = 0.13170898365871023197e1_f64 * t310 * t9369;
    let t38233 = -0.17347256376410398924e1_f64 * t33144 - 0.13170898365871023197e1_f64 * t8316 * t1659 - 0.65854491829355115987e0_f64 * t33147 - 0.8673628188205199462e0_f64 * t2146 * t7890 * t38209 * t944 + 0.26020884564615598386e1_f64 * t38215 - 0.13877805101128319139e2_f64 * t33150 + 0.52041769129231196772e1_f64 * t33153 + 0.65854491829355115987e0_f64 * t33155 + 0.69389025505641595696e1_f64 * t33157 + 0.13170898365871023197e1_f64 * t2222 * t5340 - 0.34694512752820797848e1_f64 * t38224 - t38228 + 0.8673628188205199462e0_f64 * t33170 - 0.8673628188205199462e0_f64 * t33173 + t38232;
    (t38209, t38233)
}
