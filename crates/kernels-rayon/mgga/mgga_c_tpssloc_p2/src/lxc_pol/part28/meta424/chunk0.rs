//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1600/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1600(t225: f64, t3886: f64, t3888: f64, t6889: f64, t1985: f64, t6883: f64, t6903: f64, t2379: f64, t25: f64, t2752: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t22933 = t225 * t3886;
    let t22934 = t22933 * t3888;
    let t22935 = t6889 * t22934;
    let t22936 = t1985 * t22935;
    let t22940 = t6883 * t6903;
    let t22941 = 0.38381794893125283518e-1_f64 * t22940;
    let t22951 = t25 * t2379;
    let t22960 = t2752 * t25;
    (t22934, t22935, t22936, t22940, t22941, t22951, t22960)
}
