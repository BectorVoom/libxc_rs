//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 700/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk700<F: Float>(t4581: F, t5142: F, t1154: F, t1155: F, t167: F, t1791: F, t238: F, t86: F, t1745: F, t330: F, t829: F, t304: F, t4920: F) -> (F, F, F, F, F, F) {
    let t5143 = t5142 * t4581;
    let t5147 = t1154 * t1155 * t167;
    let t5151 = t86 * t238 * t1791;
    let t5153 = t1745 * t330;
    let t5155 = t1154 * t5153 * t829;
    let t5158 = t304 * t4920;
    (t5143, t5147, t5151, t5153, t5155, t5158)
}
