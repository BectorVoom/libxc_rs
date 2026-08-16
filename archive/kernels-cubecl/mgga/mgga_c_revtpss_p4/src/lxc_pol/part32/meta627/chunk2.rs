//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2004/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2004<F: Float>(t108688: F, t1310: F, t1518: F, t18235: F, t18245: F, t2056: F, t2322: F, t27123: F, t27126: F, t28196: F, t28286: F, t28586: F, t28696: F, t28760: F, t29508: F, t30570: F, t30571: F, t30578: F, t4248: F, t4254: F, t4292: F, t651: F, t6765: F, t7359: F, t7367: F, t7373: F, t7374: F, t7378: F, t75439: F, t7732: F, t7978: F, t8065: F, t85360: F) -> F {
    let t110102 = -F::cast_from(4.0_f64) * t7359 * t18235 - F::cast_from(2.0_f64) * t18245 * t7378 - F::cast_from(2.0_f64) * t75439 * t2056 - F::cast_from(2.0_f64) * t85360 * t2056 - F::cast_from(2.0_f64) * t18245 * t7367 - F::cast_from(4.0_f64) * t651 * t8065 * t4292 - F::cast_from(4.0_f64) * t2322 * t30578 - F::cast_from(4.0_f64) * t4254 * t30578 - F::cast_from(4.0_f64) * t651 * t28586 * t1518 - F::cast_from(2.0_f64) * t651 * t6765 * t7373 - F::cast_from(2.0_f64) * t2322 * t30571 - F::cast_from(2.0_f64) * t4254 * t30571 - F::cast_from(2.0_f64) * t651 * t1310 * t30570 - F::cast_from(2.0_f64) * t29508 * t7374 - F::cast_from(4.0_f64) * t27123 * t7978 - F::cast_from(4.0_f64) * t27126 * t7978 - F::cast_from(4.0_f64) * t7732 * t28760 + F::cast_from(4.0_f64) * t28196 * t28286 * t108688 - F::cast_from(4.0_f64) * t4248 * t28696;
    t110102
}
