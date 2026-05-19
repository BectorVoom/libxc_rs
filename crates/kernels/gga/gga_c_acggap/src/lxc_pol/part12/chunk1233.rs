//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1233/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1233<F: Float>(t2385: F, t943: F, t32041: F, t36019: F, t8306: F, t32181: F, t36475: F, t38086: F, t310: F, t464: F, t9369: F, t1659: F, t2146: F, t2222: F, t33144: F, t33147: F, t33150: F, t33153: F, t33155: F, t33157: F, t33170: F, t33173: F, t5340: F, t7890: F, t8316: F, t944: F) -> (F, F) {
    let t38209 = t2385 * t943;
    let t38215 = t32041 * t8306 * t36019;
    let t38224 = t32181 * t38086 * t36475;
    let t38226 = t310 * t2385;
    let t38228 = F::cast_from(0.13170898365871023197e1_f64) * t38226 * t464;
    let t38232 = F::cast_from(0.13170898365871023197e1_f64) * t310 * t9369;
    let t38233 = -F::cast_from(0.17347256376410398924e1_f64) * t33144 - F::cast_from(0.13170898365871023197e1_f64) * t8316 * t1659 - F::cast_from(0.65854491829355115987e0_f64) * t33147 - F::cast_from(0.8673628188205199462e0_f64) * t2146 * t7890 * t38209 * t944 + F::cast_from(0.26020884564615598386e1_f64) * t38215 - F::cast_from(0.13877805101128319139e2_f64) * t33150 + F::cast_from(0.52041769129231196772e1_f64) * t33153 + F::cast_from(0.65854491829355115987e0_f64) * t33155 + F::cast_from(0.69389025505641595696e1_f64) * t33157 + F::cast_from(0.13170898365871023197e1_f64) * t2222 * t5340 - F::cast_from(0.34694512752820797848e1_f64) * t38224 - t38228 + F::cast_from(0.8673628188205199462e0_f64) * t33170 - F::cast_from(0.8673628188205199462e0_f64) * t33173 + t38232;
    (t38209, t38233)
}
