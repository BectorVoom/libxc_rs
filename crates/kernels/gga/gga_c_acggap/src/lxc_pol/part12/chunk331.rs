//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 331/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk331<F: Float>(t394: F, t441: F, t407: F, t456: F, t930: F, t955: F, t1210: F, t182: F, t119: F, t1226: F, t1228: F, t1229: F, t1231: F, t1235: F, t1238: F, t1242: F, t1246: F, t1248: F, t151: F) -> (F, F, F, F, F, F) {
    let t1251 = t394 * t441;
    let t1252 = t1251 * t407;
    let t1255 = t456 * t930;
    let t1258 = t456 * t955;
    let t1261 = t182 * t1210;
    let t1264 = t1226 - t1228 - F::cast_from(0.13170898365871023197e1_f64) * t1229 + F::cast_from(0.13170898365871023197e1_f64) * t1231 + t1235 + F::cast_from(0.13170898365871023197e1_f64) * t1238 - F::cast_from(0.13170898365871023197e1_f64) * t1242 - t1246 + F::cast_from(0.13170898365871023197e1_f64) * t151 * t1248 - F::cast_from(0.13170898365871023197e1_f64) * t151 * t1252 - F::cast_from(0.65854491829355115987e0_f64) * t151 * t1255 - F::cast_from(0.65854491829355115987e0_f64) * t151 * t1258 + F::cast_from(0.65854491829355115987e0_f64) * t119 * t1261;
    (t1251, t1252, t1255, t1258, t1261, t1264)
}
