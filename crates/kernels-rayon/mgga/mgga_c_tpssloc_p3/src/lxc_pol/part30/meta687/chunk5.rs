//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2182/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2182(t1307: f64, t22635: f64, t26331: f64, t567: f64, t6347: f64, t1985: f64, t20022: f64, t6889: f64, t6906: f64, t28192: f64, t80727: f64, t1375: f64, t1842: f64, t20029: f64, t26471: f64, t26472: f64, t26482: f64, t3887: f64, t5215: f64, t5321: f64, t6993: f64, t91487: f64, t97640: f64, t97644: f64, t97647: f64) -> f64 {
    let t97652 = t26331 * t22635 * t567 * t6347 * t1307;
    let t97658 = t1985 * t6889 * t6906 * t20022;
    let t97664 = t80727 * t28192;
    let t97666 = -2.0_f64 * t20029 * t6993 + 4.0_f64 * t5321 * t26482 + 0.16449340668482264365e-1_f64 * t97640 + 0.3289868133696452873e-1_f64 * t97644 + 0.3289868133696452873e-1_f64 * t97647 + 0.49348022005446793095e-1_f64 * t97652 - 2.0_f64 * t5215 * t26472 - 0.82246703342411321825e-2_f64 * t97658 + t91487 + 4.0_f64 * t1375 * t3887 * t26471 * t1842 - 0.11514538467937585055e0_f64 * t97664;
    t97666
}
