//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1993/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1993(t109777: f64, t109788: f64, t109798: f64, t109808: f64, t109816: f64, t109822: f64, t109829: f64, t109839: f64, t22399: f64, t26265: f64, t2027: f64, t2028: f64, t213: f64, t225: f64, t25921: f64, t26282: f64, t28890: f64, t28899: f64, t30283: f64, t543: f64, t545: f64, t561: f64, t5774: f64, t5775: f64, t6843: f64, t6919: f64, t7295: f64, t7296: f64, t7301: f64, t7506: f64, t7917: f64, t8085: f64, t96559: f64, t96561: f64, t96564: f64, t96565: f64, t96584: f64, t96591: f64) -> f64 {
    let t109842 = t109777 + t109788 + t109798 + t109808 + t109816 + t109822 + t109829 + t109839;
    let t109858 = t26265 * t22399;
    let t109864 = -0.13170898365871023197e1_f64 * t28899 * t5775 + 0.65049603595885220126e-3_f64 * t96559 - 0.13009920719177044025e-1_f64 * t96561 - t96564 + 0.19274729307122665471e-1_f64 * t96565 + 0.17347256376410398924e1_f64 * t7295 * t7296 * t8085 * t5774 - 0.4336814094102599731e0_f64 * t2027 * t2028 * t545 * t109842 - 0.8673628188205199462e0_f64 * t7917 * t28890 - 0.65854491829355115987e0_f64 * t26282 * t6919 - t96584 + 0.17347256376410398924e1_f64 * t25921 * t30283 + 0.4336814094102599731e0_f64 * t7295 * t7301 * t7506 * t6843 * t543 - 0.9757440539382783019e-2_f64 * t109858 + t96591 + 0.65854491829355115987e0_f64 * t213 * t109842 * t225 * t561;
    t109864
}
