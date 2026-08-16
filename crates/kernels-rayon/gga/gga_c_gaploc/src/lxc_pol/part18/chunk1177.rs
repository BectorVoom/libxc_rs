//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1177/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1177(t2268: f64, t2765: f64, t6474: f64, t23741: f64, t3327: f64, t10113: f64, t6305: f64, t23726: f64, t2293: f64, t7995: f64, t2343: f64, t10151: f64, t1328: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31704 = 0.85365019907028448797e-1_f64 * t2268 * t2765 * t6474;
    let t31706 = 0.28455006635676149599e-1_f64 * t23741 * t3327;
    let t31708 = 0.56910013271352299198e-1_f64 * t6305 * t10113;
    let t31710 = 0.7588001769513639893e-1_f64 * t23726 * t3327;
    let t31711 = t7995 * t2293;
    let t31714 = 0.1138200265427045984e0_f64 * t2268 * t2343 * t31711;
    let t31715 = t10151 * t1328;
    (t31704, t31706, t31708, t31710, t31711, t31714, t31715)
}
