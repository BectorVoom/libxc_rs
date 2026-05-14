//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 510/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk510<F: Float>(t1849: F, t1852: F, t1859: F, t1896: F, t1904: F, t1908: F, t1911: F, t1916: F, t1918: F, t1933: F, t1938: F, t1941: F, t1948: F, t1950: F, t1955: F, t1957: F, t1972: F, t1977: F, t1980: F, t248: F, t695: F, t704: F, t714: F, t723: F) -> (F,) {
    let t1983 = -0.310907e-1 * t1908 * t248 + 2.0 * t1911 * t704 - 2.0 * t1916 * t1918 + 1.0 * t695 * t1933 + 0.32163958997385070134e2 * t1938 * t1941 + t1849 - t1852 + t1859 - t1896 - t1904 - 0.19751673498613801407e-1 * t1948 + 0.11696447245269292414e1 * t1950 * t723 - 0.11696447245269292414e1 * t1955 * t1957 + 0.5848223622634646207e0 * t714 * t1972 + 0.17315859105681463759e2 * t1977 * t1980;
    (t1983,)
}
