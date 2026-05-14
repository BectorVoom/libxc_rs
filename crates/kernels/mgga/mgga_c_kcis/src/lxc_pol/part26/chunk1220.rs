//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1220/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1220<F: Float>(t103301: F, t1889: F, t98239: F, t1943: F, t5426: F, t833: F, t98233: F, t29404: F, t7904: F, t102398: F, t102421: F, t103251: F, t103263: F, t103328: F, t103423: F, t103496: F, t20882: F, t27369: F, t27438: F, t27453: F, t5701: F, t5709: F, t7908: F, t94227: F, t94626: F) -> (F, F, F) {
    let t103502 = t98239 * t1889 * t103301;
    let t103507 = t98233 * t5426 * t1943 * t833;
    let t103525 = t29404 * t7904;
    let t103527 = -0.46336805555555555557e-3 * t103496 + 0.66327777777777777776e-2 * t102398 - 0.92673611111111111112e-3 * t94626 * t103263 - 0.18550940104166666667e-3 * t94227 * t103502 + 0.61782407407407407408e-3 * t94626 * t103507 - 0.61836467013888888889e-4 * t94227 * t103328 + 0.46336805555555555556e-3 * t7908 * t5709 * t27453 * t20882 + 0.30918233506944444445e-4 * t27369 * t103251 - 0.30891203703703703704e-3 * t7908 * t5701 * t27438 * t20882 + 0.14739506172839506172e-2 * t102421 + 0.23168402777777777778e-3 * t7908 * t103423 - 0.22653549382716049382e-2 * t103525;
    (t103502, t103507, t103527)
}
