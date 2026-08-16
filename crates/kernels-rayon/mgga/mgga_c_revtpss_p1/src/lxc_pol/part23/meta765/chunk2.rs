//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2565/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2565(t12256: f64, t3617: f64, t3362: f64, t482: f64, t12268: f64, t1263: f64, t460: f64, t488: f64, t13181: f64, t1828: f64, t12627: f64, t12626: f64, t1769: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t56246 = t3617 * t12256;
    let t56250 = t482 * t3362;
    let t56254 = t1263 * t12268;
    let t56314 = t460 * t488;
    let t56315 = t13181 * t1828;
    let t56327 = t12627 * t488;
    let t56331 = t1769 * t12626;
    (t56246, t56250, t56254, t56314, t56315, t56327, t56331)
}
