//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 327/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk327<F: Float>(t119: F, t1226: F, t1228: F, t1229: F, t1231: F, t1235: F, t1238: F, t1242: F, t1246: F, t1248: F, t1252: F, t1255: F, t1258: F, t1261: F, t151: F) -> (F,) {
    let t1264 = t1226 - t1228 - 0.13170898365871023197e1 * t1229 + 0.13170898365871023197e1 * t1231 + t1235 + 0.13170898365871023197e1 * t1238 - 0.13170898365871023197e1 * t1242 - t1246 + 0.13170898365871023197e1 * t151 * t1248 - 0.13170898365871023197e1 * t151 * t1252 - 0.65854491829355115987e0 * t151 * t1255 - 0.65854491829355115987e0 * t151 * t1258 + 0.65854491829355115987e0 * t119 * t1261;
    (t1264,)
}
