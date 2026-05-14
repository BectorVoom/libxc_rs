//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 942/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk942<F: Float>(t11116: F, t7513: F, t1029: F, t2617: F, t7803: F, t10022: F, t10026: F, t10031: F, t10042: F, t11090: F, t11096: F, t11102: F, t11105: F, t11108: F, t11111: F, t11113: F, t1966: F, t2049: F, t2194: F, t2197: F, t3480: F, t3496: F, t3508: F, t797: F, t813: F, t833: F) -> (F, F) {
    let t11118 = 0.79445533226334281487e-1 * t7513 * t11116;
    let t11119 = t1029 * t2617;
    let t11120 = t7803 * t11119;
    let t11121 = 0.19171462976960374838e0 * t11120;
    let t11122 = -0.35750489951850426669e0 * t2049 * t3480 - 0.35750489951850426669e0 * t797 * t11090 - 0.23005755572352449806e1 * t2194 * t3496 - 0.23005755572352449806e1 * t813 * t11096 + 0.23005755572352449806e1 * t2197 * t3508 + 0.23005755572352449806e1 * t833 * t11102 + 0.30674340763136599741e1 * t833 * t11105 + t11108 - t10022 - t10026 - t10031 + t10042 - t11111 - 0.51123901271894332902e0 * t1966 * t11113 - t11118 + t11121;
    (t11119, t11122)
}
