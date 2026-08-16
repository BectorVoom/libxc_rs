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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk610;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk611;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk612;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk613;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk614;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk615;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk616;
use chunk7::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk617;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta94<F: Float>(t3: F, t40: F, t1933: F, t225: F, t344: F, t364: F, t362: F, sigma0: F, t368: F, t354: F, t1927: F, t378: F, t349: F, t381: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t1934 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk610::<F>(t3, t40);
        let t1935 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk611::<F>(t1933, t1934);
        let (t1936, t1937) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk612::<F>(t225, t344, t364);
        let t1940 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk613::<F>(t362, sigma0);
        let t1941 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk614::<F>(t1940, t368);
        let (t1942, t1945) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk615::<F>(t1941, t354, t1927, t1935, t1937, t378);
        let (t1946, t1948) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk616::<F>(t1945, t349, t225, t362);
        let t1949 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk617::<F>(t1948, t381);
    (t1934, t1935, t1936, t1937, t1940, t1941, t1942, t1945, t1946, t1948, t1949)
}
