//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 625/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk625<F: Float>(t5463: F, t79: F, t781: F, t2005: F, t2009: F, t2004: F, t1772: F, t397: F, t4889: F, t786: F, t782: F, t2015: F, t4998: F, sigma2: F) -> (F, F, F, F, F, F, F) {
    let t5464 = t79 * t5463;
    let t5465 = t5464 * t781;
    let t5468 = t2005 * t2009;
    let t5470 = t2004 * sigma2;
    let t5471 = t5470 * t1772;
    let t5477 = t397 * t4889 * t786;
    let t5479 = F::new(0.59969295720591057378e-2) * t782 * t5477;
    let t5480 = t4998 * t2015;
    (t5464, t5465, t5468, t5471, t5477, t5479, t5480)
}
