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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk618;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk619;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk620;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk621;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk622;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk623;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk624;
use chunk7::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk625;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta95<F: Float>(t1949: F, t345: F, t1945: F, t383: F, t1920: F, t353: F, t1055: F, t265: F, t394: F, t1052: F, t1923: F, t1946: F, t388: F, t1914: F, t202: F, t193: F, t870: F, t1070: F, t336: F, t25: F, t504: F, t1918: F, t40: F, t1915: F, t28: F, t1877: F, dens_threshold: F, rho0: F, zeta_threshold: F, t52: F, rho1: F, t1268: F, t1873: F, t1869: F, t191: F, t513: F, t192: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1950, t1953) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk618::<F>(t1949, t345, t1945, t383);
        let t1955 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk619::<F>(t1920, t1950, t1953, t353);
        let t1956 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk620::<F>(t1055, t1955);
        let (t1958, t1962, t1964, t1965) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk621::<F>(t265, t394, t1052, t1920, t1923, t1946, t1956, t388, t1914, t202, t193, t870, t1070, t336);
        let (t1968, t1971, t1972) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk622::<F>(t25, t265, t504, t1918, t1965, t40, t1915, t28, t1877, t1964, dens_threshold, rho0, zeta_threshold);
        let t1976 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk623::<F>(t28, t1971, t1972, t52, t1968, dens_threshold, rho1, zeta_threshold);
        let t1980 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk624::<F>(t1268, t1873, t1869);
        let (t1982, t1983) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk625::<F>(t191, t513, t192);
    (t1950, t1953, t1955, t1956, t1958, t1962, t1965, t1972, t1976, t1980, t1982, t1983)
}
