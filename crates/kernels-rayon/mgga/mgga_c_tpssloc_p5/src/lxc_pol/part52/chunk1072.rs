//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1072/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1072(t26202: f64, t6889: f64, t1985: f64, t26193: f64, t6907: f64, t225: f64, t5318: f64, t567: f64, t214: f64, t1377: f64, t1842: f64, t1307: f64) -> (f64, f64, f64, f64) {
    let t26203 = t6889 * t26202;
    let t26204 = t1985 * t26203;
    let t26206 = t26193 * t6907;
    let t26207 = t1985 * t26206;
    let t26210 = t5318 * t225 * t567;
    let t26211 = t214 * t26210;
    let t26212 = t1985 * t26211;
    let t26214 = t1377 * t1842;
    let t26215 = t26214 * t1307;
    (t26204, t26207, t26212, t26215)
}
