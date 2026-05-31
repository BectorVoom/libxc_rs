//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1015/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1015<F: Float>(t322: F, t1018: F, t3381: F, t1079: F, t2405: F, t11893: F) -> (F, F, F) {
    let t332 = F::cast_from(0.25e1_f64) < t322;
    let t11924 = t3381 * t1018;
    let t11926 = t1079 * t2405;
    let t11930 = piecewise3::<F>(t332, F::cast_from(0.0_f64), t11893);
    (t11924, t11926, t11930)
}
