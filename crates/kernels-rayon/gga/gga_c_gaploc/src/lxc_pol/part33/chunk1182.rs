//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1182/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1182(t1063: f64, t2343: f64, t6519: f64, t7995: f64, t10243: f64, t6313: f64, t6750: f64, t7930: f64, t1222: f64, t3351: f64, t2321: f64, t8289: f64, t882: f64) -> (f64, f64, f64, f64, f64) {
    let t31956 = 0.1138200265427045984e0_f64 * t1063 * t2343 * t7995 * t6519;
    let t31958 = 0.7588001769513639893e-1_f64 * t6313 * t10243;
    let t31961 = 0.17073003981405689759e0_f64 * t1063 * t7930 * t6750;
    let t31965 = t1222 * t3351;
    let t31966 = 0.31616674039640166222e-2_f64 * t31965;
    let t31968 = t882 * t8289 * t2321;
    (t31956, t31958, t31961, t31966, t31968)
}
