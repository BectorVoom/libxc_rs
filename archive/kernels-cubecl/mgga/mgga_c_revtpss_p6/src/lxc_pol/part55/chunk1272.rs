//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1272/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1272<F: Float>(t28986: F, t572: F, t7002: F, t1916: F, t32776: F, t1936: F, t2055: F, t4292: F, t1518: F, t7373: F, t34359: F, t1459: F, t34363: F) -> (F, F, F, F, F, F) {
    let t129055 = F::cast_from(6.0_f64) * t572 * t28986 * t7002;
    let t129057 = F::cast_from(6.0_f64) * t1916 * t32776;
    let t129065 = F::cast_from(6.0_f64) * t572 * t4292 * t2055 * t1936;
    let t129069 = F::cast_from(6.0_f64) * t572 * t1518 * t7373 * t1936;
    let t129072 = F::cast_from(6.0_f64) * t572 * t34359 * t7002;
    let t129078 = F::cast_from(6.0_f64) * t1459 * t34363;
    (t129055, t129057, t129065, t129069, t129072, t129078)
}
