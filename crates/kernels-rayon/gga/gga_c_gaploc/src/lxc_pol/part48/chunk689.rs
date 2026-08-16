//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 689/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk689(t13296: f64, t169: f64, t172: f64, t452: f64, t105: f64, t12771: f64, t13250: f64, t13254: f64, t13260: f64, t13264: f64, t13267: f64, t13270: f64, t13275: f64, t13279: f64, t13280: f64, t2268: f64) -> (f64, f64, f64) {
    let t13298 = t13296 * t169 * t172;
    let t13299 = t452 * t13298;
    let t13302 = -0.17073003981405689759e0_f64 * t2268 * t13250 + 0.1138200265427045984e0_f64 * t2268 * t13254 - t13260 + t13264 - t13267 + t13270 + 0.47425011059460249332e-2_f64 * t12771 + t13275 - t13279 + t13280 + 0.28455006635676149599e-1_f64 * t105 * t13299;
    (t13298, t13299, t13302)
}
