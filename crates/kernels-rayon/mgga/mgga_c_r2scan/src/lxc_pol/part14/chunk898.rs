//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 898/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk898(t2832: f64, t784: f64, t783: f64, t788: f64, t2547: f64, t6118: f64, t571: f64, t6148: f64, t2575: f64, t2579: f64, t2584: f64, t6149: f64, t6152: f64, t6509: f64, t6513: f64, t6515: f64, t6519: f64, t6538: f64, t6543: f64, t6545: f64, t8265: f64, t8268: f64, t8272: f64, t8275: f64, t8277: f64) -> f64 {
    let t8279 = t2832 * t784;
    let t8282 = 0.11643651550782197811e-1_f64 * t783 * t8279 * t788;
    let t8284 = 0.25610080155860322884e0_f64 * t6118 * t2547;
    let t8289 = t571 * t6148;
    let t8292 = -0.84755945902752848174e0_f64 * t6509 - 0.85366933852867742945e0_f64 * t6513 + 0.12805040077930161442e0_f64 * t6515 - 0.38415120233790484326e0_f64 * t6519 + 0.11643651550782197811e-1_f64 * t6538 - 0.58218257753910989057e-2_f64 * t6543 - 0.48787202696913915093e-2_f64 * t6545 - t8265 + 0.679213007128961539e-1_f64 * t8268 + 0.2037639021386884617e0_f64 * t8272 - 0.679213007128961539e-1_f64 * t8275 - 0.32927245914677557993e-1_f64 * t8277 + t8282 - t8284 + 0.86682217400542685632e-1_f64 * t6149 * t2575 + 0.2600466522016280569e0_f64 * t6152 * t2579 - 0.86682217400542685632e-1_f64 * t8289 * t2584;
    t8292
}
