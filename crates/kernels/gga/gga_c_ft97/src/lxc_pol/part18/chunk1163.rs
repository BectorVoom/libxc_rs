//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1163/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1163<F: Float>(t25798: F, t3626: F, t5578: F, t5611: F, t1632: F, t45461: F, t11247: F, t1656: F, t45169: F, t25714: F, t93324: F, t22513: F, t22572: F, t25718: F, t25708: F, t11982: F, t1742: F, t5570: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t100495 = t5578 * t3626 * t25798;
    let t100496 = t5611 * t100495;
    let t100504 = t45461 * t1632;
    let t100508 = t11247 * t1656;
    let t100512 = t45169 * t1632;
    let t100519 = t93324 * t25714;
    let t100521 = 0.10091343167942740398e-3 * t22513 * t100519;
    let t100522 = t22572 * t25718;
    let t100524 = 0.56749874115226337448e-2 * t25708 * t100522;
    let t100526 = t5570 * t1742 * t11982;
    (t100495, t100496, t100504, t100508, t100512, t100519, t100521, t100522, t100524, t100526)
}
