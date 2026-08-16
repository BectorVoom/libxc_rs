//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 761/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk761(t11726: f64, t740: f64, t748: f64, t1929: f64, t5060: f64, t5286: f64, t11450: f64, t747: f64, t746: f64, t1948: f64, t10479: f64, t7303: f64, sigma2: f64) -> (f64, f64, f64, f64) {
    let t11727 = t11726 * t740;
    let t11728 = t11727 * t748;
    let t11730 = t1929 * t5060;
    let t11731 = t11730 * sigma2;
    let t11732 = t11731 * t5286;
    let t11734 = t747 * t11450;
    let t11735 = t746 * t11734;
    let t11736 = t1948 * t11735;
    let t11738 = t7303 * t10479;
    (t11728, t11732, t11736, t11738)
}
