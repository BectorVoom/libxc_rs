//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 695/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk695(t12894: f64, t4540: f64, t12762: f64, t1457: f64, t1572: f64, t12766: f64, t12866: f64, t12870: f64, t12873: f64, t12877: f64, t12880: f64, t12883: f64, t12884: f64, t12889: f64, t12893: f64) -> (f64, f64, f64) {
    let t12896 = 0.21450293971110256001e1_f64 * t4540 * t12894;
    let t12897 = t1457 * t12762;
    let t12898 = t1572 * t12897;
    let t12900 = t1457 * t12766;
    let t12902 = 0.71500979903700853338e0_f64 * t1572 * t12900;
    let t12903 = 0.23005755572352449806e2_f64 * t12866 + t12870 - t12873 + t12877 - t12880 - t12883 - 0.21450293971110256002e1_f64 * t12884 - t12889 + t12893 - t12896 + 0.14300195980740170668e1_f64 * t12898 + t12902;
    (t12897, t12900, t12903)
}
