//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta95 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk618;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk619;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk620;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk621;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk622;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk623;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta95(t265: f64, t394: f64, t1052: f64, t1920: f64, t1923: f64, t1946: f64, t1956: f64, t388: f64, t1914: f64, t202: f64, t193: f64, t870: f64, t1070: f64, t336: f64, t25: f64, t504: f64, t1918: f64, t40: f64, t1915: f64, t28: f64, t1877: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t52: f64, rho1: f64, t1268: f64, t1873: f64, t1869: f64, t191: f64, t513: f64, t192: f64, t209: f64, t540: f64, t1878: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1958, t1962, t1964, t1965) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk618(t265, t394, t1052, t1920, t1923, t1946, t1956, t388, t1914, t202, t193, t870, t1070, t336);
        let (t1968, t1971, t1972) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk619(t25, t265, t504, t1918, t1965, t40, t1915, t28, t1877, t1964, dens_threshold, rho0, zeta_threshold);
        let t1976 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk620(t28, t1971, t1972, t52, t1968, dens_threshold, rho1, zeta_threshold);
        let t1980 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk621(t1268, t1873, t1869);
        let (t1982, t1983) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk622(t191, t513, t192);
        let (t1984, t1985) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk623(t209, t540, t1878);
    (t1958, t1962, t1965, t1972, t1976, t1980, t1982, t1983, t1984, t1985)
}
