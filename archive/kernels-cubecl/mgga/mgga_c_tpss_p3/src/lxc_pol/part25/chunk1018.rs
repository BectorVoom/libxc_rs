//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1018/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1018<F: Float>(t45: F, t4802: F, t823: F, t4573: F, t8050: F, t2225: F, t4579: F, t13335: F, t3431: F, t3575: F, t581: F, t78: F, t8061: F, zeta_threshold: F) -> (F, F, F) {
    let t151 = t45 <= zeta_threshold;
    let t14080 = t4802 * t823;
    let t14084 = t8050 * t4573;
    let t14089 = t2225 * t4579;
    let t14095 = piecewise3::<F>(t151, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t14084 * t581 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t3575 * t3431 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t14089 * t581 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t78 * t13335);
    let t14096 = t8061 * t4573;
    (t14080, t14095, t14096)
}
