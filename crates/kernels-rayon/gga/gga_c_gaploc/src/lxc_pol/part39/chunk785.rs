//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 785/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk785(t13745: f64, t13759: f64, t209: f64, t12032: f64, t921: f64, t12860: f64, t2355: f64, t3718: f64, t1382: f64, t12866: f64, t12870: f64, t12873: f64, t12877: f64, t12880: f64, t12883: f64, t12884: f64, t12889: f64, t12893: f64, t12896: f64, t12898: f64, t12902: f64, t12906: f64, t12909: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13760 = t13745 + t13759;
    let t13761 = t13760 * t209;
    let t13762 = t12032 * t921;
    let t13763 = 2.0_f64 * t12860;
    let t13764 = t2355 * t3718;
    let t13765 = t3718 * t921;
    let t13766 = t1382 * t13765;
    let t13767 = 2.0_f64 * t13766;
    let t13772 = 0.11502877786176224903e2_f64 * t12866 + t12870 - t12873 + t12877 - t12880 - t12883 - 0.10725146985555128001e1_f64 * t12884 - t12889 + t12893 - t12896 + 0.71500979903700853338e0_f64 * t12898 + t12902 - 0.46011511144704899612e1_f64 * t12906 + t12909;
    (t13760, t13761, t13762, t13763, t13764, t13765, t13767, t13772)
}
