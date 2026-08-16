//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 695/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk695(t11110: f64, t1890: f64, t3431: f64, t590: f64, t10628: f64, t4820: f64, t7513: f64, t1029: f64, t2617: f64, t7803: f64, t10022: f64, t10026: f64, t10031: f64, t10042: f64, t11090: f64, t11096: f64, t11102: f64, t11105: f64, t11108: f64, t1966: f64, t2049: f64, t2194: f64, t2197: f64, t3480: f64, t3496: f64, t3508: f64, t797: f64, t813: f64, t833: f64) -> (f64, f64, f64, f64, f64) {
    let t11111 = 0.19171462976960374838e0_f64 * t11110;
    let t11112 = t1890 * t3431;
    let t11113 = t11112 * t590;
    let t11116 = t4820 * t10628;
    let t11118 = 0.79445533226334281487e-1_f64 * t7513 * t11116;
    let t11119 = t1029 * t2617;
    let t11120 = t7803 * t11119;
    let t11121 = 0.19171462976960374838e0_f64 * t11120;
    let t11122 = -0.35750489951850426669e0_f64 * t2049 * t3480 - 0.35750489951850426669e0_f64 * t797 * t11090 - 0.23005755572352449806e1_f64 * t2194 * t3496 - 0.23005755572352449806e1_f64 * t813 * t11096 + 0.23005755572352449806e1_f64 * t2197 * t3508 + 0.23005755572352449806e1_f64 * t833 * t11102 + 0.30674340763136599741e1_f64 * t833 * t11105 + t11108 - t10022 - t10026 - t10031 + t10042 - t11111 - 0.51123901271894332902e0_f64 * t1966 * t11113 - t11118 + t11121;
    (t11111, t11112, t11118, t11121, t11122)
}
