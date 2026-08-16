//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta93 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk604;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk605;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk606;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk607;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk608;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk609;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk610;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta93(t1921: f64, t387: f64, t345: f64, t131: f64, t1919: f64, t350: f64, t365: f64, t335: f64, t371: f64, t3: f64, t40: f64, t225: f64, t344: f64, t364: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1922 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk604(t1921, t387);
        let (t1923, t1926) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk605(t1922, t345, t131, t1919);
        let (t1927, t1929, t1930, t1932) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk606(t1926, t350, t365, t335, t371);
        let t1933 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk607(t1930, t1932);
        let t1934 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk608(t3, t40);
        let t1935 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk609(t1933, t1934);
        let (t1936, t1937) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk610(t225, t344, t364);
    (t1922, t1923, t1926, t1927, t1929, t1930, t1932, t1933, t1934, t1935, t1936, t1937)
}
