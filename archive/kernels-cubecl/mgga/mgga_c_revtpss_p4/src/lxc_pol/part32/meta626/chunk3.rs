//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1993/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1993<F: Float>(t109777: F, t109788: F, t109798: F, t109808: F, t109816: F, t109822: F, t109829: F, t109839: F, t22399: F, t26265: F, t2027: F, t2028: F, t213: F, t225: F, t25921: F, t26282: F, t28890: F, t28899: F, t30283: F, t543: F, t545: F, t561: F, t5774: F, t5775: F, t6843: F, t6919: F, t7295: F, t7296: F, t7301: F, t7506: F, t7917: F, t8085: F, t96559: F, t96561: F, t96564: F, t96565: F, t96584: F, t96591: F) -> F {
    let t109842 = t109777 + t109788 + t109798 + t109808 + t109816 + t109822 + t109829 + t109839;
    let t109858 = t26265 * t22399;
    let t109864 = -F::cast_from(0.13170898365871023197e1_f64) * t28899 * t5775 + F::cast_from(0.65049603595885220126e-3_f64) * t96559 - F::cast_from(0.13009920719177044025e-1_f64) * t96561 - t96564 + F::cast_from(0.19274729307122665471e-1_f64) * t96565 + F::cast_from(0.17347256376410398924e1_f64) * t7295 * t7296 * t8085 * t5774 - F::cast_from(0.4336814094102599731e0_f64) * t2027 * t2028 * t545 * t109842 - F::cast_from(0.8673628188205199462e0_f64) * t7917 * t28890 - F::cast_from(0.65854491829355115987e0_f64) * t26282 * t6919 - t96584 + F::cast_from(0.17347256376410398924e1_f64) * t25921 * t30283 + F::cast_from(0.4336814094102599731e0_f64) * t7295 * t7301 * t7506 * t6843 * t543 - F::cast_from(0.9757440539382783019e-2_f64) * t109858 + t96591 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t109842 * t225 * t561;
    t109864
}
