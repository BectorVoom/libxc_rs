//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 529/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk529(t447: f64, t9193: f64, t2343: f64, t1437: f64, t3158: f64, t2304: f64, t2349: f64, t3118: f64, t535: f64, t3087: f64, t1063: f64, t2268: f64, t3088: f64, t3119: f64, t419: f64, t9207: f64, t9210: f64, t9212: f64, t9216: f64) -> (f64, f64) {
    let t9219 = t9193 * t447;
    let t9220 = t2343 * t9219;
    let t9223 = t3158 * t1437;
    let t9226 = t2304 * t2349;
    let t9229 = t535 * t3118;
    let t9232 = t535 * t3087;
    let t9239 = -t9207 + t9210 + 0.28455006635676149599e-1_f64 * t1063 * t9212 + 0.1138200265427045984e0_f64 * t2268 * t9216 - 0.56910013271352299198e-1_f64 * t1063 * t9220 + 0.85365019907028448797e-1_f64 * t1063 * t9223 - 0.17073003981405689759e0_f64 * t2268 * t9226 + 0.28455006635676149599e-1_f64 * t2268 * t9229 + 0.28455006635676149599e-1_f64 * t2268 * t9232 + 0.28455006635676149599e-1_f64 * t419 * t3088 + 0.28455006635676149599e-1_f64 * t419 * t3119;
    (t9219, t9239)
}
