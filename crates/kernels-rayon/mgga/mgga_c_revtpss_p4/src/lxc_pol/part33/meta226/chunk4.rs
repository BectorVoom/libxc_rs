//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1031/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1031(t1580: f64, t213: f64, t2437: f64, t2443: f64, t2460: f64, t2473: f64, t257: f64, t4323: f64, t4326: f64, t4474: f64, t4478: f64, t4482: f64, t6042: f64, t6049: f64, t6072: f64, t865: f64) -> f64 {
    let t6075 = t2437 - t2443 - 0.10975748638225852664e-1_f64 * t4323 + 0.10975748638225852664e-1_f64 * t4478 + t2460 + 0.19514881078765566038e-1_f64 * t4326 - 0.19514881078765566038e-1_f64 * t4482 - t2473 + 0.65854491829355115987e0_f64 * t213 * t6042 * t257 - 0.13170898365871023197e1_f64 * t4474 * t1580 + 0.13170898365871023197e1_f64 * t865 * t6049 - 0.65854491829355115987e0_f64 * t865 * t6072;
    t6075
}
