//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta95 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk618;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk619;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk620;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk621;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk622;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk623;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk624;
use chunk7::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk625;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta95(t1949: f64, t345: f64, t1945: f64, t383: f64, t1920: f64, t353: f64, t1055: f64, t265: f64, t394: f64, t1052: f64, t1923: f64, t1946: f64, t388: f64, t1914: f64, t202: f64, t193: f64, t870: f64, t1070: f64, t336: f64, t25: f64, t504: f64, t1918: f64, t40: f64, t1915: f64, t28: f64, t1877: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t52: f64, rho1: f64, t1268: f64, t1873: f64, t1869: f64, t191: f64, t513: f64, t192: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1950, t1953) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk618(t1949, t345, t1945, t383);
        let t1955 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk619(t1920, t1950, t1953, t353);
        let t1956 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk620(t1055, t1955);
        let (t1958, t1962, t1964, t1965) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk621(t265, t394, t1052, t1920, t1923, t1946, t1956, t388, t1914, t202, t193, t870, t1070, t336);
        let (t1968, t1971, t1972) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk622(t25, t265, t504, t1918, t1965, t40, t1915, t28, t1877, t1964, dens_threshold, rho0, zeta_threshold);
        let t1976 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk623(t28, t1971, t1972, t52, t1968, dens_threshold, rho1, zeta_threshold);
        let t1980 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk624(t1268, t1873, t1869);
        let (t1982, t1983) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk625(t191, t513, t192);
    (t1950, t1953, t1955, t1956, t1958, t1962, t1965, t1972, t1976, t1980, t1982, t1983)
}
