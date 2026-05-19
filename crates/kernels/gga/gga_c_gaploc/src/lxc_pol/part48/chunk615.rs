//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 615/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk615<F: Float>(t11110: F, t1890: F, t3431: F, t590: F, t10628: F, t4820: F, t7513: F, t1029: F, t2617: F, t7803: F, t10022: F, t10026: F, t10031: F, t10042: F, t11090: F, t11096: F, t11102: F, t11105: F, t11108: F, t1966: F, t2049: F, t2194: F, t2197: F, t3480: F, t3496: F, t3508: F, t797: F, t813: F, t833: F) -> (F, F, F) {
    let t11111 = F::cast_from(0.19171462976960374838e0_f64) * t11110;
    let t11112 = t1890 * t3431;
    let t11113 = t11112 * t590;
    let t11116 = t4820 * t10628;
    let t11118 = F::cast_from(0.79445533226334281487e-1_f64) * t7513 * t11116;
    let t11119 = t1029 * t2617;
    let t11120 = t7803 * t11119;
    let t11121 = F::cast_from(0.19171462976960374838e0_f64) * t11120;
    let t11122 = -F::cast_from(0.35750489951850426669e0_f64) * t2049 * t3480 - F::cast_from(0.35750489951850426669e0_f64) * t797 * t11090 - F::cast_from(0.23005755572352449806e1_f64) * t2194 * t3496 - F::cast_from(0.23005755572352449806e1_f64) * t813 * t11096 + F::cast_from(0.23005755572352449806e1_f64) * t2197 * t3508 + F::cast_from(0.23005755572352449806e1_f64) * t833 * t11102 + F::cast_from(0.30674340763136599741e1_f64) * t833 * t11105 + t11108 - t10022 - t10026 - t10031 + t10042 - t11111 - F::cast_from(0.51123901271894332902e0_f64) * t1966 * t11113 - t11118 + t11121;
    (t11112, t11120, t11122)
}
