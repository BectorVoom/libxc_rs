//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3859/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3859<F: Float>(t13716: F, t13768: F, t13867: F, t13892: F, t13911: F, t13914: F, t13917: F, t1395: F, t1877: F, t1879: F, t22223: F, t22229: F, t22236: F, t225: F, t3889: F, t4049: F, t4050: F, t4053: F, t539: F, t541: F, t5644: F, t5650: F, t5651: F, t5655: F, t6832: F, t73345: F, t74099: F, t74100: F, t74102: F, t74103: F, t74113: F, t74127: F, t74140: F, t74152: F) -> F {
    let t74165 = -F::new(24.0) * t5650 * t5651 * t13716 + F::new(240.0) * t5650 * t13768 * t13867 - F::new(12.0) * t6832 * t4050 + F::new(12.0) * t5644 * t5655 - F::new(48.0) * t22229 * t13911 - F::new(24.0) * t22229 * t13914 + F::new(6.0) * t1877 * t13917 + F::new(6.0) * t13892 * t1879 + F::new(60.0) * t5650 * t22236 * t3889 - (t74099 + t74100 + t74102 + t74103 + t74113 + t74127 + t74140 + t74152) * t225 * t541 + F::new(3.0) * t6832 * t4053 - F::new(24.0) * t539 * t4049 * t73345 + F::new(6.0) * t22223 * t1395;
    t74165
}
