//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1305/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1305(t1445: f64, t2087: f64, t24926: f64, t935: f64, t10820: f64, t10914: f64, t2089: f64, t539: f64, t16036: f64, t6111: f64, t2028: f64, t28593: f64, t28633: f64, t28636: f64, t33376: f64, t33381: f64, t33385: f64, t33387: f64, t33389: f64, t33392: f64, t33394: f64, t33397: f64, t33399: f64) -> f64 {
    let t33405 = 0.69017266717057349418e1_f64 * t2087 * t1445 * t24926 * t935;
    let t33409 = 0.28600391961480341335e1_f64 * t10914 * t539 * t2089 * t10820;
    let t33412 = 0.57200783922960682671e1_f64 * t6111 * t16036 * t10820;
    let t33413 = t28593 + t33376 + t33381 - t33385 + t33387 - t33389 + t33392 + t33394 - t33397 - 0.79445533226334281486e-1_f64 * t33399 * t2028 - t33405 - t33409 + t33412 - t28633 + t28636;
    t33413
}
