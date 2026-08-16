//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1982/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1982(t87401: f64, t87403: f64, t87405: f64, t87411: f64, t81887: f64, t81889: f64, t81899: f64, t81903: f64, t81909: f64, t81912: f64, t87379: f64, t87381: f64, t87387: f64, t87389: f64, t87391: f64, t87395: f64, t87399: f64, t87409: f64) -> f64 {
    let t92675 = 7.0_f64 / 576.0_f64 * t87401;
    let t92676 = 119.0_f64 / 3456.0_f64 * t87403;
    let t92677 = 0.10541775202358879834e-2_f64 * t87405;
    let t92679 = 0.56521858531796547194e-2_f64 * t87411;
    let t92682 = t87379 / 192.0_f64 + t87381 / 192.0_f64 - 7.0_f64 / 576.0_f64 * t81887 + 7.0_f64 / 1152.0_f64 * t81889 + 0.40372756094140390853e-3_f64 * t81899 + 0.40372756094140390852e-3_f64 * t81903 - 0.63250651214153279003e-2_f64 * t87387 + 5.0_f64 / 96.0_f64 * t87389 + 5.0_f64 / 192.0_f64 * t87391 + 0.48447307312968469024e-2_f64 * t87395 + 0.24223653656484234512e-2_f64 * t87399 - t92675 + t92676 - t92677 - 0.13565246047631171326e0_f64 * t87409 + t92679 + 0.28260929265898273597e-2_f64 * t81909 - 0.45217486825437237756e-1_f64 * t81912;
    t92682
}
