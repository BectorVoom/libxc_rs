//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1311/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1311(t10882: f64, t1391: f64, t2684: f64, t11061: f64, t14549: f64, t32356: f64, t5241: f64, t5640: f64, t590: f64, t1890: f64, t1966: f64, t32435: f64) -> (f64, f64, f64, f64) {
    let t33507 = t2684 * t1391 * t10882;
    let t33508 = 0.2698205900461089792e0_f64 * t33507;
    let t33518 = 0.30674340763136599742e1_f64 * t14549 * t11061;
    let t33522 = 0.30674340763136599742e1_f64 * t5640 * t5241 * t32356 * t590;
    let t33526 = 0.51123901271894332902e1_f64 * t1966 * t1890 * t32435 * t590;
    (t33508, t33518, t33522, t33526)
}
