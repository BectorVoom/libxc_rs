//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 711/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk711(t14271: f64, t2343: f64, t105: f64, t13260: f64, t13264: f64, t13267: f64, t13270: f64, t13275: f64, t13279: f64, t13280: f64, t13303: f64, t13741: f64, t14268: f64, t2268: f64) -> (f64, f64) {
    let t14272 = t2343 * t14271;
    let t14275 = -0.47425011059460249332e-2_f64 * t13741 - t13260 + t13264 - t13267 + t13270 - 0.28455006635676149599e-1_f64 * t105 * t14268 + t13275 - t13279 + t13280 + t13303 + 0.1138200265427045984e0_f64 * t2268 * t14272;
    (t14272, t14275)
}
