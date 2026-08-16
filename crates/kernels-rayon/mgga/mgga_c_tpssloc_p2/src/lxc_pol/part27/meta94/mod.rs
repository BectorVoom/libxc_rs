//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta94 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk610;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk611;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk612;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk613;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk614;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk615;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk616;
use chunk7::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk617;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta94(t3: f64, t40: f64, t1933: f64, t225: f64, t344: f64, t364: f64, t362: f64, sigma0: f64, t368: f64, t354: f64, t1927: f64, t378: f64, t349: f64, t381: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1934 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk610(t3, t40);
        let t1935 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk611(t1933, t1934);
        let (t1936, t1937) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk612(t225, t344, t364);
        let t1940 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk613(t362, sigma0);
        let t1941 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk614(t1940, t368);
        let (t1942, t1945) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk615(t1941, t354, t1927, t1935, t1937, t378);
        let (t1946, t1948) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk616(t1945, t349, t225, t362);
        let t1949 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk617(t1948, t381);
    (t1934, t1935, t1936, t1937, t1940, t1941, t1942, t1945, t1946, t1948, t1949)
}
