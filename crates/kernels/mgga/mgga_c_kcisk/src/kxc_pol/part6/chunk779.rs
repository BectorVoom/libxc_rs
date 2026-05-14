//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 779/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk779<F: Float>(t10642: F, t28371: F, t28375: F, t28383: F, t28391: F, t28410: F, t28412: F, t28415: F, t28417: F, t28420: F, t28423: F, t28426: F, t28431: F, t28435: F, t28492: F, t1676: F, t1685: F) -> (F, F) {
    let t28506 = -0.82785e-1 * t28410 - 0.3883875e1 * t28412 - t10642 - 0.412621875e-1 * t28415 + 0.19419375e1 * t28417 - 0.36793333333333333333e-1 * t28420 - 0.82785e-1 * t28423 - 0.49671e0 * t28426 + 0.12077e1 * t28375 - 0.181155e1 * t28383 + 0.16557e0 * t28431 - 0.33547222222222222222e0 * t28371 - 0.301925e0 * t28391 + 0.16504875e0 * t28435;
    let t28507 = t28492 + t28506;
    let t28509 = t1676 * t28507 * t1685;
    (t28507, t28509)
}
