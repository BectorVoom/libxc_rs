//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 575/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk575<F: Float>(t10094: F, t106: F, t316: F, t3270: F, t773: F, t10069: F, t10077: F, t10080: F, t10083: F, t10086: F, t10089: F, t2009: F, t2087: F, t2103: F, t2194: F, t2197: F, t3304: F, t3315: F, t3319: F, t5782: F, t6060: F, t7712: F, t780: F, t813: F, t833: F, t955: F) -> F {
    let t10095 = t10094 * t106;
    let t10096 = t10095 * t316;
    let t10099 = t773 * t3270;
    let t10102 = -F::new(0.23005755572352449806e1) * t2194 * t3304 - F::new(0.23005755572352449806e1) * t813 * t10069 - F::new(0.69017266717057349418e1) * t5782 * t3315 + F::new(0.23005755572352449806e1) * t2197 * t3319 + F::new(0.23005755572352449806e1) * t833 * t10077 - F::new(0.62115540045351614476e2) * t2087 * t10080 - F::new(0.21450293971110256001e1) * t6060 * t10083 + F::new(0.71500979903700853338e0) * t2103 * t10086 - F::new(0.35750489951850426669e0) * t10089 * t2009 + F::new(0.71500979903700853338e0) * t955 * t7712 + F::new(0.35750489951850426669e0) * t780 * t10096 - F::new(0.35750489951850426669e0) * t10099 * t2009;
    t10102
}
