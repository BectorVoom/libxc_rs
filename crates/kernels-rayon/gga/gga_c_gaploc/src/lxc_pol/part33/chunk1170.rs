//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1170/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1170(t23726: f64, t3327: f64, t2293: f64, t7995: f64, t2268: f64, t2343: f64, t10145: f64, t6313: f64, t6776: f64, t988: f64, t25893: f64, t6520: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31710 = 0.7588001769513639893e-1_f64 * t23726 * t3327;
    let t31711 = t7995 * t2293;
    let t31714 = 0.1138200265427045984e0_f64 * t2268 * t2343 * t31711;
    let t31724 = 0.15176003539027279786e0_f64 * t6313 * t10145;
    let t31727 = 0.28455006635676149599e-1_f64 * t2268 * t6776 * t988;
    let t31735 = t25893 * t6520;
    (t31710, t31711, t31714, t31724, t31727, t31735)
}
