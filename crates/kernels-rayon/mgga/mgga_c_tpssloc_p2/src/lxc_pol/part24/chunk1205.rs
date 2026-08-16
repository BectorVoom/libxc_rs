//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1205/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1205(t1949: f64, t2966: f64, t1920: f64, t1948: f64, t3166: f64, t345: f64, t6680: f64, t6781: f64, t6805: f64, t968: f64, t210: f64, t6795: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t23617 = t2966 * t1949;
    let t23619 = 0.18277045187202515961e-2_f64 * t1920 * t23617;
    let t23620 = t1948 * t3166;
    let t23621 = t345 * t23620;
    let t23626 = t6680 * t6781;
    let t23628 = t968 * t6805;
    let t23629 = t1920 * t23628;
    let t23631 = t6795 * t210;
    (t23617, t23619, t23620, t23621, t23626, t23628, t23629, t23631)
}
