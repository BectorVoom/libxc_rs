//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta93 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk604;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk605;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk606;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk607;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk608;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk609;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta93(t1915: f64, t25: f64, t1877: f64, t337: f64, t38: f64, t1887: f64, t225: f64, t381: f64, t387: f64, t345: f64, t131: f64, t350: f64, t365: f64, t335: f64, t371: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1918, t1919, t1920) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk604(t1915, t25, t1877, t337, t38, t1887);
        let t1921 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk605(t225, t381);
        let t1922 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk606(t1921, t387);
        let (t1923, t1926) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk607(t1922, t345, t131, t1919);
        let (t1927, t1929, t1930, t1932) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk608(t1926, t350, t365, t335, t371);
        let t1933 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk609(t1930, t1932);
    (t1918, t1919, t1920, t1921, t1922, t1923, t1926, t1927, t1929, t1930, t1932, t1933)
}
