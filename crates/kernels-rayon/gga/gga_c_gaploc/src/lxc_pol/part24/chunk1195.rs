//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1195/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1195(t1063: f64, t21042: f64, t2765: f64, t25955: f64, t894: f64, t20013: f64, t2268: f64, t2854: f64, t6320: f64, t2343: f64, t6519: f64, t7995: f64) -> (f64, f64, f64, f64) {
    let t31945 = 0.85365019907028448797e-1_f64 * t1063 * t2765 * t21042;
    let t31948 = 0.28455006635676149599e-1_f64 * t1063 * t894 * t25955;
    let t31952 = 0.17073003981405689759e0_f64 * t2268 * t6320 * t2854 * t20013;
    let t31956 = 0.1138200265427045984e0_f64 * t1063 * t2343 * t7995 * t6519;
    (t31945, t31948, t31952, t31956)
}
