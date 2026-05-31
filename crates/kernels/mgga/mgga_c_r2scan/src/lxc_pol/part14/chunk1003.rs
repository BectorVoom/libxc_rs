//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1003/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1003<F: Float>(t11338: F, t3579: F, t3465: F, t797: F, t495: F, t1146: F, t2881: F, t3718: F, t498: F) -> (F, F, F, F, F, F, F, F) {
    let t12049 = t3579 * t11338;
    let t12050 = t12049 / F::cast_from(4.0_f64);
    let t12051 = t3465 * t797;
    let t12052 = t495 * t12051;
    let t12053 = t3579 * t12052;
    let t12054 = t12053 / F::cast_from(4.0_f64);
    let t12055 = t1146 * t2881;
    let t12056 = t498 * t3718;
    (t12049, t12050, t12051, t12052, t12053, t12054, t12055, t12056)
}
