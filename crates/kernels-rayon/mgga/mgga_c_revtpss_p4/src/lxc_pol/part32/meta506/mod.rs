//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta506 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1794;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1795;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta506(t6880: f64, t7271: f64, t6856: f64, t6876: f64, t7264: f64, t26017: f64, t6850: f64, t26028: f64, t6871: f64, t6884: f64, t7252: f64, t25983: f64, t6864: f64, t1955: f64, t6888: f64, t1882: f64, t1903: f64, t543: f64, t1868: f64, t1907: f64, t1501: f64, t1518: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t30037, t30039, t30041, t30043, t30045, t30048, t30050) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1794(t6880, t7271, t6856, t6876, t7264, t26017, t6850, t26028, t6871, t6884, t7252, t25983, t6864);
        let (t30071, t30105, t30122, t30138) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1795(t1955, t6888, t1882, t1903, t543, t1868, t1907, t1501, t1518);
    (t30037, t30039, t30041, t30043, t30045, t30048, t30050, t30071, t30105, t30122, t30138)
}
