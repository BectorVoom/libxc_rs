//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2101/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2101<F: Float>(t13730: F, t2023: F, t2782: F, t10073: F, t25938: F, t27836: F, t14079: F, t26054: F, t7289: F, t97925: F, t1882: F, t25921: F, t25930: F, t25931: F, t25933: F, t26036: F, t27853: F, t27972: F, t7917: F, t94716: F, t94779: F, t94784: F, t94799: F, t94803: F, t94807: F, t94811: F, t94813: F) -> F {
    let t98001 = F::cast_from(0.21951497276451705328e-1_f64) * t2782 * t2023 * t13730;
    let t98003 = t10073 * t27836 * t25938;
    let t98010 = F::cast_from(0.19514881078765566038e-1_f64) * t26054 * t14079;
    let t98011 = t7289 * t97925;
    let t98022 = -F::cast_from(0.68540937416128198416e-1_f64) * t94779 - F::cast_from(0.17347256376410398924e1_f64) * t25930 * t94716 * t27972 - t98001 - F::cast_from(0.24093411633903331839e-3_f64) * t98003 + F::cast_from(0.8673628188205199462e0_f64) * t25921 * t27853 - F::cast_from(0.4336814094102599731e0_f64) * t7917 * t26036 + t94784 - t98010 + F::cast_from(0.17135234354032049604e-1_f64) * t98011 - F::cast_from(0.17347256376410398924e1_f64) * t25930 * t25931 * t1882 * t25933 - F::cast_from(0.9757440539382783019e-2_f64) * t94799 + F::cast_from(0.25702851531048074406e-1_f64) * t94803 + F::cast_from(0.34270468708064099208e-2_f64) * t94807 + F::cast_from(0.72280234901709995518e-2_f64) * t94811 + F::cast_from(0.51405703062096148812e-1_f64) * t94813;
    t98022
}
