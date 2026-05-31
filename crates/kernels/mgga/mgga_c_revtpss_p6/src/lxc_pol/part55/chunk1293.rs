//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1293/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1293<F: Float>(t128920: F, t128930: F, t128932: F, t128933: F, t2055: F, t2089: F, t2322: F, t28707: F, t28734: F, t28737: F, t29337: F, t29422: F, t33287: F, t33311: F, t34821: F, t4254: F, t4297: F, t651: F, t7474: F, t7586: F, t7732: F, t8152: F, t8764: F) -> F {
    let t131064 = -F::cast_from(2.0_f64) * t2055 * t29337 * t651 - t2089 * t29422 - F::cast_from(2.0_f64) * t2322 * t34821 - t28707 * t8764 - F::cast_from(2.0_f64) * t28734 * t7586 - F::cast_from(2.0_f64) * t28737 * t7586 - F::cast_from(2.0_f64) * t33287 * t4297 - F::cast_from(2.0_f64) * t33311 * t7732 - F::cast_from(2.0_f64) * t34821 * t4254 - t7474 * t8152 - t128920 - t128930 - t128932 - t128933;
    t131064
}
