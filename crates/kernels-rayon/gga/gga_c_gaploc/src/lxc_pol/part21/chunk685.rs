//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 685/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk685(t447: f64, t6417: f64, t1064: f64, t105: f64, t1063: f64, t1079: f64, t1083: f64, t1212: f64, t2264: f64, t2268: f64, t2269: f64, t2296: f64, t2350: f64, t380: f64, t3818: f64, t3833: f64, t419: f64, t6305: f64, t6313: f64, t6342: f64, t6345: f64, t6348: f64, t6353: f64, t6356: f64, t6396: f64, t877: f64, t889: f64) -> (f64, f64) {
    let t6418 = t6417 * t447;
    let t6419 = t1064 * t6418;
    let t6422 = -0.85365019907028448797e-1_f64 * t2268 * t6342 + 0.28455006635676149599e-1_f64 * t2268 * t6345 + 0.56910013271352299198e-1_f64 * t2268 * t6348 + 0.56910013271352299198e-1_f64 * t6305 * t2269 - 0.56910013271352299198e-1_f64 * t1063 * t6353 - 0.28455006635676149599e-1_f64 * t1063 * t6356 - 0.7588001769513639893e-1_f64 * t3818 * t2264 + 0.28455006635676149599e-1_f64 * t105 * t6396 + 0.12646669615856066488e-1_f64 * t1079 * t877 - 0.12646669615856066488e-1_f64 * t1079 * t889 + 0.7588001769513639893e-1_f64 * t1083 * t877 + 0.7588001769513639893e-1_f64 * t380 * t2296 + 0.28455006635676149599e-1_f64 * t1212 * t877 + 0.56910013271352299198e-1_f64 * t419 * t2296 - 0.56910013271352299198e-1_f64 * t419 * t2350 + 0.7588001769513639893e-1_f64 * t6313 * t2269 - 0.56910013271352299198e-1_f64 * t3833 * t2264 + 0.56910013271352299198e-1_f64 * t1063 * t6419;
    (t6418, t6422)
}
