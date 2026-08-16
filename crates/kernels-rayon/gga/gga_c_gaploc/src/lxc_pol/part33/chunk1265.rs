//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1265/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1265(t1445: f64, t2087: f64, t3009: f64, t7112: f64, t10882: f64, t1391: f64, t2684: f64, t11061: f64, t14549: f64, t32356: f64, t5241: f64, t5640: f64, t590: f64) -> (f64, f64, f64, f64) {
    let t33505 = 0.69017266717057349418e1_f64 * t2087 * t1445 * t3009 * t7112;
    let t33507 = t2684 * t1391 * t10882;
    let t33508 = 0.2698205900461089792e0_f64 * t33507;
    let t33518 = 0.30674340763136599742e1_f64 * t14549 * t11061;
    let t33522 = 0.30674340763136599742e1_f64 * t5640 * t5241 * t32356 * t590;
    (t33505, t33508, t33518, t33522)
}
