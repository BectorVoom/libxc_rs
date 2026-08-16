//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1187/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1187(t23741: f64, t3355: f64, t10141: f64, t6305: f64, t2787: f64, t6393: f64, t2268: f64, t2343: f64, t10122: f64, t1323: f64, t10185: f64, t1358: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31863 = 0.56910013271352299198e-1_f64 * t23741 * t3355;
    let t31865 = 0.1138200265427045984e0_f64 * t6305 * t10141;
    let t31866 = t2787 * t6393;
    let t31869 = 0.56910013271352299198e-1_f64 * t2268 * t2343 * t31866;
    let t31870 = t10122 * t1323;
    let t31878 = t1358 * t10185;
    (t31863, t31865, t31866, t31869, t31870, t31878)
}
