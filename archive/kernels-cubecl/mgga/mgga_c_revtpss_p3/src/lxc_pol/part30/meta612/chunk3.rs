//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2098/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2098<F: Float>(t13756: F, t7271: F, t13760: F, t25972: F, t94424: F, t94430: F, t94444: F, t94449: F, t98135: F, t98141: F, t98145: F, t98147: F, t98148: F, t98152: F) -> F {
    let t98154 = t7271 * t13756;
    let t98156 = t25972 * t13760;
    let t98157 = F::cast_from(0.2032800112371413129e-3_f64) * t98156;
    let t98158 = F::cast_from(0.34299214494455789578e-2_f64) * t98135 + F::cast_from(0.2032800112371413129e-3_f64) * t94424 - F::cast_from(0.16006300097412701803e-1_f64) * t94430 + F::cast_from(0.2168320119862840671e-2_f64) * t94444 + F::cast_from(0.14291339372689912324e-4_f64) * t94449 - F::cast_from(0.15244095330869239812e-3_f64) * t98141 + t98145 + t98147 + F::cast_from(0.10841600599314203355e-2_f64) * t98148 - F::cast_from(0.57165357490759649296e-3_f64) * t98152 - F::cast_from(0.17149607247227894789e-2_f64) * t98154 - t98157;
    t98158
}
