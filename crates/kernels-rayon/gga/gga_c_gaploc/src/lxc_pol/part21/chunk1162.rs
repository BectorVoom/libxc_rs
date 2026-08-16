//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1162/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1162(t10242: f64, t1624: f64, t2268: f64, t10116: f64, t6305: f64, t6313: f64, t23726: f64, t3355: f64, t10256: f64, t590: f64, t23759: f64, t10241: f64, t161: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31525 = 0.28455006635676149599e-1_f64 * t2268 * t1624 * t10242;
    let t31527 = 0.56910013271352299198e-1_f64 * t6305 * t10116;
    let t31533 = 0.7588001769513639893e-1_f64 * t6313 * t10116;
    let t31539 = 0.15176003539027279786e0_f64 * t23726 * t3355;
    let t31540 = t590 * t10256;
    let t31542 = 0.12646669615856066488e-1_f64 * t23759 * t31540;
    let t31543 = t10241 * t161;
    (t31525, t31527, t31533, t31539, t31540, t31542, t31543)
}
