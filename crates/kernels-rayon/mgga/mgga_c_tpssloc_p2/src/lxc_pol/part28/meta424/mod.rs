//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta424 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1600;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1601;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta424(t225: f64, t3886: f64, t3888: f64, t6889: f64, t1985: f64, t6883: f64, t6903: f64, t2379: f64, t25: f64, t2752: f64, t13487: f64, t606: f64, t776: f64, t2553: f64, t1887: f64, t6581: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22934, t22935, t22936, t22940, t22941, t22951, t22960) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1600(t225, t3886, t3888, t6889, t1985, t6883, t6903, t2379, t25, t2752);
        let (t22961, t22964, t22968, t22986) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1601(t13487, t22960, t606, t776, t25, t2553, t1887, t6581);
    (t22934, t22935, t22936, t22940, t22941, t22951, t22960, t22961, t22964, t22968, t22986)
}
