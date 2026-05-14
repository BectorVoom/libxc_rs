//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1403/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1403<F: Float>(t22132: F, t22134: F, t22139: F, t22141: F, t22143: F, t22148: F, t22152: F, t22154: F, t22156: F, t22158: F, t22161: F, t22164: F, t22167: F, t406: F, t7794: F, t1871: F, t2782: F, t584: F) -> (F, F, F) {
    let t26552 = -0.93505639170679904297e3 * t22132 - 0.42107210082969452691e2 * t22134 + t22139 + 0.4051561992e0 * t22141 + 0.27324781257645766813e6 * t22143 + t22148 + t22152 + 0.1350520664e0 * t22154 + 0.4051561992e0 * t22156 + 0.4051561992e0 * t22158 + 0.127022098e-1 * t22161 - 0.12154685976e1 * t22164 - 0.24309371952e1 * t22167;
    let t26555 = t406 * t7794;
    let t26556 = 12.0 * t26555;
    let t26560 = t584 * t2782 * t1871;
    (t26552, t26556, t26560)
}
