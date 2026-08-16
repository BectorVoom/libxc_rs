//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 502/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk502<F: Float>(t447: F, t9193: F, t2343: F, t1437: F, t3158: F, t2304: F, t2349: F, t3118: F, t535: F, t3087: F, t1063: F, t2268: F, t3088: F, t3119: F, t419: F, t9207: F, t9210: F, t9212: F, t9216: F) -> (F, F) {
    let t9219 = t9193 * t447;
    let t9220 = t2343 * t9219;
    let t9223 = t3158 * t1437;
    let t9226 = t2304 * t2349;
    let t9229 = t535 * t3118;
    let t9232 = t535 * t3087;
    let t9239 = -t9207 + t9210 + F::cast_from(0.28455006635676149599e-1_f64) * t1063 * t9212 + F::cast_from(0.1138200265427045984e0_f64) * t2268 * t9216 - F::cast_from(0.56910013271352299198e-1_f64) * t1063 * t9220 + F::cast_from(0.85365019907028448797e-1_f64) * t1063 * t9223 - F::cast_from(0.17073003981405689759e0_f64) * t2268 * t9226 + F::cast_from(0.28455006635676149599e-1_f64) * t2268 * t9229 + F::cast_from(0.28455006635676149599e-1_f64) * t2268 * t9232 + F::cast_from(0.28455006635676149599e-1_f64) * t419 * t3088 + F::cast_from(0.28455006635676149599e-1_f64) * t419 * t3119;
    (t9219, t9239)
}
