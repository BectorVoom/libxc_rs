//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2098/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2098(t13756: f64, t7271: f64, t13760: f64, t25972: f64, t94424: f64, t94430: f64, t94444: f64, t94449: f64, t98135: f64, t98141: f64, t98145: f64, t98147: f64, t98148: f64, t98152: f64) -> f64 {
    let t98154 = t7271 * t13756;
    let t98156 = t25972 * t13760;
    let t98157 = 0.2032800112371413129e-3_f64 * t98156;
    let t98158 = 0.34299214494455789578e-2_f64 * t98135 + 0.2032800112371413129e-3_f64 * t94424 - 0.16006300097412701803e-1_f64 * t94430 + 0.2168320119862840671e-2_f64 * t94444 + 0.14291339372689912324e-4_f64 * t94449 - 0.15244095330869239812e-3_f64 * t98141 + t98145 + t98147 + 0.10841600599314203355e-2_f64 * t98148 - 0.57165357490759649296e-3_f64 * t98152 - 0.17149607247227894789e-2_f64 * t98154 - t98157;
    t98158
}
