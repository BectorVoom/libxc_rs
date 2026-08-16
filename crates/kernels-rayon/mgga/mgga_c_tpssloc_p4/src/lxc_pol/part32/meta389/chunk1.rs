//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1471/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1471(t16949: f64, t2701: f64, t820: f64, t2697: f64, t5628: f64, t210: f64, t5567: f64, t776: f64, t1495: f64, t4119: f64, t5571: f64, t13223: f64, t5591: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16951 = t2701 * t820 * t16949;
    let t16954 = t2697 * t5628;
    let t16957 = t210 * t5567 * t776;
    let t16961 = t210 * t1495 * t4119;
    let t16965 = t210 * t5571 * t776;
    let t16968 = t13223 * t5591;
    (t16951, t16954, t16957, t16961, t16965, t16968)
}
