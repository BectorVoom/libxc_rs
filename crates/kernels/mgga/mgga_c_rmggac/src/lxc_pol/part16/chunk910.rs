//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 910/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk910<F: Float>(t2010: F, t38835: F, t8465: F, t2415: F, t38820: F, t7349: F, t2329: F, t38973: F, t118: F, t128: F, t2001: F, t6261: F, t675: F) -> (F, F, F, F) {
    let t45152 = t2010 * t8465 * t38835;
    let t45155 = t7349 * t2415 * t38820;
    let t45158 = t38973 * t2329;
    let t45163 = t675 * t2001 * t118 * t128 * t6261;
    (t45152, t45155, t45158, t45163)
}
