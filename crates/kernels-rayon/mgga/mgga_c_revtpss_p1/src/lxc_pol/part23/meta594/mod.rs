//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta594 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2236;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta594(t23842: f64, t4806: f64, t1042: f64, t23633: f64, t4801: f64, t1651: f64, t5825: f64, t4872: f64, t1592: f64, t19649: f64, t1015: f64, t22671: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23843, t23844, t23847, t23848, t23851, t23852, t23857, t23858, t23859, t23862, t23863, t23868) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2236(t23842, t4806, t1042, t23633, t4801, t1651, t5825, t4872, t1592, t19649, t1015, t22671);
    (t23843, t23844, t23847, t23848, t23851, t23852, t23857, t23858, t23859, t23862, t23863, t23868)
}
