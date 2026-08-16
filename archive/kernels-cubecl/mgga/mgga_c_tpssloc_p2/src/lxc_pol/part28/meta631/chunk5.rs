//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1982/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1982<F: Float>(t87401: F, t87403: F, t87405: F, t87411: F, t81887: F, t81889: F, t81899: F, t81903: F, t81909: F, t81912: F, t87379: F, t87381: F, t87387: F, t87389: F, t87391: F, t87395: F, t87399: F, t87409: F) -> F {
    let t92675 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t87401;
    let t92676 = F::cast_from(119.0_f64) / F::cast_from(3456.0_f64) * t87403;
    let t92677 = F::cast_from(0.10541775202358879834e-2_f64) * t87405;
    let t92679 = F::cast_from(0.56521858531796547194e-2_f64) * t87411;
    let t92682 = t87379 / F::cast_from(192.0_f64) + t87381 / F::cast_from(192.0_f64) - F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t81887 + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t81889 + F::cast_from(0.40372756094140390853e-3_f64) * t81899 + F::cast_from(0.40372756094140390852e-3_f64) * t81903 - F::cast_from(0.63250651214153279003e-2_f64) * t87387 + F::cast_from(5.0_f64) / F::cast_from(96.0_f64) * t87389 + F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t87391 + F::cast_from(0.48447307312968469024e-2_f64) * t87395 + F::cast_from(0.24223653656484234512e-2_f64) * t87399 - t92675 + t92676 - t92677 - F::cast_from(0.13565246047631171326e0_f64) * t87409 + t92679 + F::cast_from(0.28260929265898273597e-2_f64) * t81909 - F::cast_from(0.45217486825437237756e-1_f64) * t81912;
    t92682
}
