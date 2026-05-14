//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 620/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk620<F: Float>(t12806: F, t6320: F, t2268: F, t3148: F, t988: F, t12792: F, t169: F, t172: F, t452: F, t105: F, t12764: F, t12769: F, t12771: F, t12774: F, t12794: F, t12799: F, t12802: F, t12805: F) -> (F, F, F, F, F) {
    let t12807 = t6320 * t12806;
    let t12809 = 0.17073003981405689759e0 * t2268 * t12807;
    let t12810 = t3148 * t988;
    let t12812 = 0.28455006635676149599e-1 * t2268 * t12810;
    let t12814 = t12792 * t169 * t172;
    let t12815 = t452 * t12814;
    let t12818 = 0.1138200265427045984e0 * t12764 + t12769 + 0.23712505529730124666e-2 * t12771 - 0.1707300398140568976e0 * t12774 - 0.28455006635676149599e-1 * t105 * t12794 + t12799 - t12802 + t12805 - t12809 + t12812 + 0.28455006635676149599e-1 * t105 * t12815;
    (t12807, t12810, t12814, t12815, t12818)
}
