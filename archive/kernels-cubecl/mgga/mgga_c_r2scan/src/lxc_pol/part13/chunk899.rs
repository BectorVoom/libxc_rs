//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 899/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk899<F: Float>(t2832: F, t784: F, t783: F, t788: F, t2547: F, t6118: F, t571: F, t6148: F, t2575: F, t2579: F, t2584: F, t6149: F, t6152: F, t6509: F, t6513: F, t6515: F, t6519: F, t6538: F, t6543: F, t6545: F, t8265: F, t8268: F, t8272: F, t8275: F, t8277: F) -> F {
    let t8279 = t2832 * t784;
    let t8282 = F::cast_from(0.11643651550782197811e-1_f64) * t783 * t8279 * t788;
    let t8284 = F::cast_from(0.25610080155860322884e0_f64) * t6118 * t2547;
    let t8289 = t571 * t6148;
    let t8292 = -F::cast_from(0.84755945902752848174e0_f64) * t6509 - F::cast_from(0.85366933852867742945e0_f64) * t6513 + F::cast_from(0.12805040077930161442e0_f64) * t6515 - F::cast_from(0.38415120233790484326e0_f64) * t6519 + F::cast_from(0.11643651550782197811e-1_f64) * t6538 - F::cast_from(0.58218257753910989057e-2_f64) * t6543 - F::cast_from(0.48787202696913915093e-2_f64) * t6545 - t8265 + F::cast_from(0.679213007128961539e-1_f64) * t8268 + F::cast_from(0.2037639021386884617e0_f64) * t8272 - F::cast_from(0.679213007128961539e-1_f64) * t8275 - F::cast_from(0.32927245914677557993e-1_f64) * t8277 + t8282 - t8284 + F::cast_from(0.86682217400542685632e-1_f64) * t6149 * t2575 + F::cast_from(0.2600466522016280569e0_f64) * t6152 * t2579 - F::cast_from(0.86682217400542685632e-1_f64) * t8289 * t2584;
    t8292
}
