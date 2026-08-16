//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1978/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1978<F: Float>(t28780: F, t97700: F, t6861: F, t7506: F, t1364: F, t30248: F, t786: F, t102329: F, t102339: F, t102346: F, t102661: F, t108206: F, t1444: F, t2097: F, t22252: F, t25930: F, t26079: F, t26304: F, t27837: F, t27864: F, t28863: F, t30071: F, t30247: F, t4003: F, t543: F, t7295: F, t7296: F, t7301: F, t7532: F, t94823: F, t96380: F, t96382: F) -> (F, F) {
    let t109567 = t97700 * t28780;
    let t109573 = t7506 * t6861;
    let t109579 = t786 * t30248 * t1364;
    let t109598 = -F::cast_from(0.28912093960683998207e-1_f64) * t109567 - t102329 + F::cast_from(0.8673628188205199462e0_f64) * t7295 * t7296 * t30247 * t1444 - t102339 - F::cast_from(0.8673628188205199462e0_f64) * t7295 * t26079 * t109573 * t4003 + F::cast_from(0.9757440539382783019e-2_f64) * t109579 - F::cast_from(0.8673628188205199462e0_f64) * t25930 * t26304 * t108206 - t102346 + F::cast_from(0.52041769129231196772e1_f64) * t94823 * t102661 * t27864 + F::cast_from(0.4336814094102599731e0_f64) * t7295 * t7301 * t2097 * t22252 * t543 + F::cast_from(0.17135234354032049604e-2_f64) * t96380 + F::cast_from(0.17135234354032049604e-2_f64) * t96382 - F::cast_from(0.4336814094102599731e0_f64) * t30071 * t7532 + F::cast_from(0.17347256376410398924e1_f64) * t27837 * t28863;
    (t109573, t109598)
}
