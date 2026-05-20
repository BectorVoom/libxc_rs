//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3111/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3111<F: Float>(t13069: F, t5265: F, t1260: F, t17332: F, t12916: F, t17747: F, t17749: F, t11231: F, t1266: F, t12787: F, t12828: F, t17261: F, t17265: F, t17729: F, t20921: F, t44403: F, t44405: F, t44409: F, t44411: F, t44415: F, t44418: F, t45710: F, t5330: F, t5343: F, t5391: F, t57164: F, t57167: F, t57170: F, t57173: F, t57176: F) -> F {
    let t57178 = t13069 * t5265;
    let t57187 = t17332 * t1260;
    let t57191 = t17747 * t12916 * t17749;
    let t57193 = F::cast_from(0.45732285992607719436e-2_f64) * t5391 * t12828 + F::cast_from(0.12862205435420921092e-2_f64) * t17261 * t17265 + F::cast_from(0.95275595817932748827e-3_f64) * t44403 - F::cast_from(0.28582678745379824648e-3_f64) * t44405 - F::cast_from(0.95275595817932748827e-4_f64) * t44409 - F::cast_from(0.57165357490759649295e-3_f64) * t44411 - F::cast_from(0.57165357490759649295e-3_f64) * t44415 - F::cast_from(0.28582678745379824648e-3_f64) * t44418 - F::cast_from(0.57165357490759649295e-3_f64) * t57164 - F::cast_from(0.57165357490759649295e-3_f64) * t57167 - F::cast_from(0.28582678745379824648e-3_f64) * t57170 + F::cast_from(0.17149607247227894789e-2_f64) * t57173 - F::cast_from(0.57165357490759649295e-3_f64) * t57176 + F::cast_from(0.42874018118069736972e-3_f64) * t57178 - F::cast_from(0.71456696863449561621e-3_f64) * t17729 * t12787 * t20921 * t11231 + F::cast_from(0.12862205435420921092e-2_f64) * t45710 * t5330 * t5343 - F::cast_from(0.42874018118069736972e-3_f64) * t57187 * t1266 - F::cast_from(0.25724410870841842183e-2_f64) * t57191;
    t57193
}
