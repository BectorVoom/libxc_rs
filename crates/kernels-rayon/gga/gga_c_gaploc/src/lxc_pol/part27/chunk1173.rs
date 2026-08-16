//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1173/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1173(t31694: f64, t1365: f64, t25740: f64, t6525: f64, t2268: f64, t2349: f64, t7930: f64, t2765: f64, t6474: f64, t23741: f64, t3327: f64, t10113: f64, t6305: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31695 = 0.23712505529730124666e-2_f64 * t31694;
    let t31697 = t6525 * t1365 * t25740;
    let t31698 = 0.11856252764865062333e-2_f64 * t31697;
    let t31701 = 0.17073003981405689759e0_f64 * t2268 * t7930 * t2349;
    let t31704 = 0.85365019907028448797e-1_f64 * t2268 * t2765 * t6474;
    let t31706 = 0.28455006635676149599e-1_f64 * t23741 * t3327;
    let t31708 = 0.56910013271352299198e-1_f64 * t6305 * t10113;
    (t31695, t31698, t31701, t31704, t31706, t31708)
}
