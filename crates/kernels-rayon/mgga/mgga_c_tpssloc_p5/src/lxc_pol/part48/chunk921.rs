//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 921/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk921(t2006: f64, t212: f64, t22642: f64, t6890: f64, t3886: f64, t6992: f64, t1385: f64, t1992: f64, t22635: f64, t1985: f64, t22904: f64, t6889: f64, t6906: f64) -> (f64, f64, f64) {
    let t113941 = 0.16449340668482264365e-1_f64 * t22642 * t212 * t2006 * t6890;
    let t113946 = t3886 * t6992;
    let t113950 = 0.6579736267392905746e-1_f64 * t1992 * t22635 * t113946 * t1385;
    let t113956 = 0.16449340668482264365e-1_f64 * t1985 * t6889 * t6906 * t22904;
    (t113941, t113950, t113956)
}
