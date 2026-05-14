//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1054/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1054<F: Float>(t31623: F, t493: F, t10257: F, t3833: F, t10217: F, t10224: F, t105: F, t1079: F, t1212: F, t12963: F, t1358: F, t1359: F, t169: F, t172: F, t31581: F, t31584: F, t31589: F, t31594: F, t31600: F, t3341: F, t3359: F, t380: F, t419: F, t452: F, t488: F, t492: F) -> (F, F) {
    let t31624 = t493 * t31623;
    let t31646 = 0.1138200265427045984e0 * t3833 * t10257;
    let t31647 = t31581 + t31584 - t31589 - t31594 - 0.63233348079280332442e-2 * t1358 * t1359 * t12963 * t488 - t31600 - 0.28455006635676149599e-1 * t105 * t492 * t31624 - 0.7588001769513639893e-1 * t380 * t10217 - 0.28455006635676149599e-1 * t1212 * t3359 - 0.56910013271352299198e-1 * t419 * t10217 + 0.56910013271352299198e-1 * t419 * t10224 + 0.28455006635676149599e-1 * t105 * t452 * t31623 * t169 * t172 + 0.28455006635676149599e-1 * t1212 * t3341 + 0.12646669615856066488e-1 * t1079 * t3341 - t31646;
    (t31624, t31647)
}
