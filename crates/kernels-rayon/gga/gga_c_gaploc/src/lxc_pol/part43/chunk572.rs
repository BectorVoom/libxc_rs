//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 572/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk572(t10094: f64, t106: f64, t316: f64, t3270: f64, t773: f64, t10069: f64, t10077: f64, t10080: f64, t10083: f64, t10086: f64, t10089: f64, t2009: f64, t2087: f64, t2103: f64, t2194: f64, t2197: f64, t3304: f64, t3315: f64, t3319: f64, t5782: f64, t6060: f64, t7712: f64, t780: f64, t813: f64, t833: f64, t955: f64) -> f64 {
    let t10095 = t10094 * t106;
    let t10096 = t10095 * t316;
    let t10099 = t773 * t3270;
    let t10102 = -0.23005755572352449806e1_f64 * t2194 * t3304 - 0.23005755572352449806e1_f64 * t813 * t10069 - 0.69017266717057349418e1_f64 * t5782 * t3315 + 0.23005755572352449806e1_f64 * t2197 * t3319 + 0.23005755572352449806e1_f64 * t833 * t10077 - 0.62115540045351614476e2_f64 * t2087 * t10080 - 0.21450293971110256001e1_f64 * t6060 * t10083 + 0.71500979903700853338e0_f64 * t2103 * t10086 - 0.35750489951850426669e0_f64 * t10089 * t2009 + 0.71500979903700853338e0_f64 * t955 * t7712 + 0.35750489951850426669e0_f64 * t780 * t10096 - 0.35750489951850426669e0_f64 * t10099 * t2009;
    t10102
}
