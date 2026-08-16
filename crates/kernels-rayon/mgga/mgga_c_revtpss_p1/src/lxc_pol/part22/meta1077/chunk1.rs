//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3859/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3859(t13716: f64, t13768: f64, t13867: f64, t13892: f64, t13911: f64, t13914: f64, t13917: f64, t1395: f64, t1877: f64, t1879: f64, t22223: f64, t22229: f64, t22236: f64, t225: f64, t3889: f64, t4049: f64, t4050: f64, t4053: f64, t539: f64, t541: f64, t5644: f64, t5650: f64, t5651: f64, t5655: f64, t6832: f64, t73345: f64, t74099: f64, t74100: f64, t74102: f64, t74103: f64, t74113: f64, t74127: f64, t74140: f64, t74152: f64) -> f64 {
    let t74165 = -24.0_f64 * t5650 * t5651 * t13716 + 240.0_f64 * t5650 * t13768 * t13867 - 12.0_f64 * t6832 * t4050 + 12.0_f64 * t5644 * t5655 - 48.0_f64 * t22229 * t13911 - 24.0_f64 * t22229 * t13914 + 6.0_f64 * t1877 * t13917 + 6.0_f64 * t13892 * t1879 + 60.0_f64 * t5650 * t22236 * t3889 - (t74099 + t74100 + t74102 + t74103 + t74113 + t74127 + t74140 + t74152) * t225 * t541 + 3.0_f64 * t6832 * t4053 - 24.0_f64 * t539 * t4049 * t73345 + 6.0_f64 * t22223 * t1395;
    t74165
}
