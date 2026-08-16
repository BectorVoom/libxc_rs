//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1172/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1172(t31623: f64, t493: f64, t10257: f64, t3833: f64, t10217: f64, t10224: f64, t105: f64, t1079: f64, t1212: f64, t12963: f64, t1358: f64, t1359: f64, t169: f64, t172: f64, t31581: f64, t31584: f64, t31589: f64, t31594: f64, t31600: f64, t3341: f64, t3359: f64, t380: f64, t419: f64, t452: f64, t488: f64, t492: f64) -> (f64, f64) {
    let t31624 = t493 * t31623;
    let t31646 = 0.1138200265427045984e0_f64 * t3833 * t10257;
    let t31647 = t31581 + t31584 - t31589 - t31594 - 0.63233348079280332442e-2_f64 * t1358 * t1359 * t12963 * t488 - t31600 - 0.28455006635676149599e-1_f64 * t105 * t492 * t31624 - 0.7588001769513639893e-1_f64 * t380 * t10217 - 0.28455006635676149599e-1_f64 * t1212 * t3359 - 0.56910013271352299198e-1_f64 * t419 * t10217 + 0.56910013271352299198e-1_f64 * t419 * t10224 + 0.28455006635676149599e-1_f64 * t105 * t452 * t31623 * t169 * t172 + 0.28455006635676149599e-1_f64 * t1212 * t3341 + 0.12646669615856066488e-1_f64 * t1079 * t3341 - t31646;
    (t31624, t31647)
}
