//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta92 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk589;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk590;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk591;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk592;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk593;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta92<F: Float>(t1963: F, t30: F, t1940: F, t343: F, t43: F, t136: F, t359: F, t365: F, sigma0: F, t351: F, t348: F, t375: F, t225: F, t385: F, t338: F, t993: F, t378: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1966, t1967, t1968, t1971, t1972) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk589::<F>(t1963, t30, t1940, t343, t43, t136, t359, t365, sigma0);
        let (t1973, t1976) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk590::<F>(t1972, t351, t1968, t348, t375);
        let (t1977, t1978) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk591::<F>(t1976, t225, t385);
        let (t1981, t1982) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk592::<F>(t338, t993);
        let t1983 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk593::<F>(t1982, t378);
    (t1966, t1967, t1968, t1971, t1972, t1973, t1976, t1977, t1978, t1981, t1982, t1983)
}
