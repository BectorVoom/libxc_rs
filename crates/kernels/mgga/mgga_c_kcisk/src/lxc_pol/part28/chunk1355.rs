//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1355/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1355<F: Float>(t23768: F, t9663: F, t1799: F, t34107: F, t6981: F, t116223: F, t6680: F, t116145: F, t1869: F, t34221: F, t116197: F, t116211: F, t116245: F, t116409: F, t116482: F, t116489: F, t117248: F, t34013: F, t34018: F, t34078: F, t34148: F, t34192: F, t34225: F, t9672: F) -> (F, F, F, F, F) {
    let t121156 = t9663 * t23768;
    let t121171 = t1799 * t34107 * t6981;
    let t121174 = t1799 * t116223 * t6680;
    let t121181 = t1869 * t116145 * t34221;
    let t121183 = -0.46296296296296296297e-2 * t116197 + 0.10185185185185185186e0 * t121156 * t9672 + t116211 - 0.24125000000000000001e-1 * t34192 * t34078 - 0.46561250000000000002e-2 * t116409 * t34078 - 0.23148148148148148149e-2 * t116245 + 0.21444444444444444446e-1 * t34225 * t34148 + 0.64333333333333333337e-1 * t34225 * t34078 + 0.12416333333333333334e-1 * t117248 * t34078 + 0.22109259259259259259e-2 * t121171 - 0.33163888888888888888e-2 * t121174 - 0.71481481481481481483e-2 * t116489 * t34013 + 0.24691358024691358025e-1 * t116482 * t34018 + 0.88437037037037037033e-2 * t121181;
    (t121156, t121171, t121174, t121181, t121183)
}
