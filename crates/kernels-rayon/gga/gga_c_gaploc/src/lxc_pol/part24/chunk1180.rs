//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1180/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1180(t10340: f64, t1328: f64, t10145: f64, t6313: f64, t2268: f64, t6776: f64, t988: f64, t2343: f64, t31688: f64, t31690: f64, t31692: f64, t31695: f64, t31698: f64, t31701: f64, t31704: f64, t31706: f64, t31708: f64, t31710: f64, t31714: f64, t31715: f64, t6320: f64) -> (f64, f64) {
    let t31719 = t10340 * t1328;
    let t31724 = 0.15176003539027279786e0_f64 * t6313 * t10145;
    let t31727 = 0.28455006635676149599e-1_f64 * t2268 * t6776 * t988;
    let t31728 = -t31688 - t31690 - t31692 - t31695 + t31698 - t31701 - t31704 + t31706 + t31708 + t31710 + t31714 + 0.34146007962811379518e0_f64 * t2268 * t2343 * t31715 - 0.17073003981405689759e0_f64 * t2268 * t6320 * t31719 + t31724 + t31727;
    (t31719, t31728)
}
