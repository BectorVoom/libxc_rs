//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 497/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk497<F: Float>(t3118: F, t535: F, t3087: F, t1063: F, t2268: F, t3088: F, t3119: F, t419: F, t9207: F, t9210: F, t9212: F, t9216: F, t9220: F, t9223: F, t9226: F, t105: F, t1358: F, t9062: F, t9067: F, t9072: F, t9077: F, t9080: F, t9085: F, t9089: F, t9092: F, t9094: F, t9130: F, t9158: F, t9203: F) -> (F,) {
    let t9229 = t535 * t3118;
    let t9232 = t535 * t3087;
    let t9239 = -t9207 + t9210 + 0.28455006635676149599e-1 * t1063 * t9212 + 0.1138200265427045984e0 * t2268 * t9216 - 0.56910013271352299198e-1 * t1063 * t9220 + 0.85365019907028448797e-1 * t1063 * t9223 - 0.17073003981405689759e0 * t2268 * t9226 + 0.28455006635676149599e-1 * t2268 * t9229 + 0.28455006635676149599e-1 * t2268 * t9232 + 0.28455006635676149599e-1 * t419 * t3088 + 0.28455006635676149599e-1 * t419 * t3119;
    let t9241 = -0.31616674039640166221e-2 * t1358 * t9062 - 0.31616674039640166221e-2 * t1358 * t9067 - t9072 + t9077 + 0.94850022118920498663e-2 * t1358 * t9080 + t9085 - t9089 + t9092 - t9094 + 0.28455006635676149599e-1 * t105 * t9130 + t9158 + t9203 + t9239;
    (t9241,)
}
