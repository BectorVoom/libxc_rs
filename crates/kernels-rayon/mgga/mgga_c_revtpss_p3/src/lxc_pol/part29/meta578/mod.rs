//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta578 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1928;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1929;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta578(t2689: f64, t27239: f64, t25277: f64, t4458: f64, t14685: f64, t14756: f64, t7021: f64, t14760: f64, t93015: f64, t2723: f64, t836: f64, t886: f64, t1955: f64, t27198: f64, t2769: f64, t25309: f64, t2453: f64, t27212: f64, t1032: f64, t4469: f64, t867: f64, t786: f64, t1559: f64, t2771: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t99091, t99099, t99102, t99113, t99155) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1928(t2689, t27239, t25277, t4458, t14685, t14756, t7021, t14760, t93015, t2723, t836, t886);
        let (t99191, t99237, t99257, t99270, t99271, t99272, t99277) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1929(t1955, t27198, t2769, t25309, t2453, t27212, t1032, t4469, t867, t786, t1559, t2771);
    (t99091, t99099, t99102, t99113, t99155, t99191, t99237, t99257, t99270, t99271, t99272, t99277)
}
