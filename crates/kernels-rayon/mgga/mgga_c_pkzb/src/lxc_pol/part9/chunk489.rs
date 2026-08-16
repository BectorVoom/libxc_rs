//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 489/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk489(t1849: f64, t1852: f64, t1859: f64, t1896: f64, t1904: f64, t1908: f64, t1911: f64, t1916: f64, t1918: f64, t1933: f64, t1938: f64, t1941: f64, t1948: f64, t1950: f64, t1955: f64, t1957: f64, t1972: f64, t1977: f64, t1980: f64, t248: f64, t695: f64, t704: f64, t714: f64, t723: f64) -> f64 {
    let t1983 = -0.310907e-1_f64 * t1908 * t248 + 2.0_f64 * t1911 * t704 - 2.0_f64 * t1916 * t1918 + 1.0_f64 * t695 * t1933 + 0.32163958997385070134e2_f64 * t1938 * t1941 + t1849 - t1852 + t1859 - t1896 - t1904 - 0.19751673498613801407e-1_f64 * t1948 + 0.11696447245269292414e1_f64 * t1950 * t723 - 0.11696447245269292414e1_f64 * t1955 * t1957 + 0.5848223622634646207e0_f64 * t714 * t1972 + 0.17315859105681463759e2_f64 * t1977 * t1980;
    t1983
}
