//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1824/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1824(t14586: f64, t2722: f64, t2645: f64, t231: f64, t50511: f64, t198: f64, t2394: f64, t2246: f64, t4171: f64, t10308: f64, t1466: f64, t13267: f64, t602: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t51574 = t14586 * t2722;
    let t51608 = t14586 * t2645;
    let t51698 = t50511 * t231;
    let t51780 = t198 * t2394;
    let t60221 = t4171 * t2246;
    let t60224 = t1466 * t10308;
    let t60248 = t13267 * t602;
    (t51574, t51608, t51698, t51780, t60221, t60224, t60248)
}
