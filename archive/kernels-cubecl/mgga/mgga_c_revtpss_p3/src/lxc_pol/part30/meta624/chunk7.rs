//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2157/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2157<F: Float>(t25277: F, t4458: F, t14685: F, t14756: F, t7021: F, t14760: F, t93015: F, t93067: F, t93069: F, t93073: F, t93077: F, t93080: F, t93084: F, t93086: F, t93088: F, t93091: F, t93095: F) -> F {
    let t99099 = t25277 * t4458;
    let t99100 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t99099;
    let t99102 = t7021 * t14685 * t14756;
    let t99103 = F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t99102;
    let t99113 = t93015 * t14760;
    let t99116 = t99100 - t99103 - F::cast_from(0.90702367218671976886e-1_f64) * t93067 + F::cast_from(0.80031500487063509016e-2_f64) * t93069 + F::cast_from(0.2168320119862840671e-2_f64) * t93073 - F::cast_from(0.10164000561857065645e-3_f64) * t93077 + F::cast_from(0.14291339372689912324e-4_f64) * t93080 - F::cast_from(0.28582678745379824648e-4_f64) * t93084 - F::cast_from(0.40015750243531754508e-1_f64) * t93086 - F::cast_from(0.30488190661738479624e-3_f64) * t93088 + F::cast_from(0.14291339372689912324e-4_f64) * t93091 - F::cast_from(0.90357964994909313586e-5_f64) * t99113 + F::cast_from(0.50820002809285328225e-3_f64) * t93095;
    t99116
}
