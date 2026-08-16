//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1570/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1570(t22643: f64, t6890: f64, t22642: f64, t225: f64, t3879: f64, t567: f64, t214: f64, t1985: f64, t3911: f64, t6906: f64, t6889: f64, t1372: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22644 = t22643 * t6890;
    let t22645 = t22642 * t22644;
    let t22648 = t3879 * t225 * t567;
    let t22649 = t214 * t22648;
    let t22650 = t1985 * t22649;
    let t22662 = t6906 * t3911;
    let t22663 = t6889 * t22662;
    let t22664 = t1985 * t22663;
    let t22666 = t214 * t1372;
    (t22644, t22645, t22648, t22649, t22650, t22662, t22663, t22664, t22666)
}
