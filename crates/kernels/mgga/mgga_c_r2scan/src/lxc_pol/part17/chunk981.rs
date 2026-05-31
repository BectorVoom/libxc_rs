//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 981/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk981<F: Float>(t12045: F, t3262: F, t3781: F, t885: F, t11338: F, t3579: F, t3465: F, t797: F, t495: F, t1146: F, t2881: F, t3718: F, t498: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12046 = t3262 * t12045;
    let t12047 = F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t12046;
    let t12048 = t3781 * t885;
    let t12049 = t3579 * t11338;
    let t12050 = t12049 / F::cast_from(4.0_f64);
    let t12051 = t3465 * t797;
    let t12052 = t495 * t12051;
    let t12053 = t3579 * t12052;
    let t12054 = t12053 / F::cast_from(4.0_f64);
    let t12055 = t1146 * t2881;
    let t12056 = t498 * t3718;
    (t12046, t12047, t12048, t12049, t12050, t12052, t12053, t12054, t12055, t12056)
}
