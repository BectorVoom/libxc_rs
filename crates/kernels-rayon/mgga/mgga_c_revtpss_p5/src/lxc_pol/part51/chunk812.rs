//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 812/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk812(t2457: f64, t25945: f64, t25944: f64, t1426: f64, t25920: f64, t7063: f64, t7286: f64, t2470: f64, t7285: f64, t7289: f64, t3974: f64, t7259: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25946 = t25945 * t2457;
    let t25948 = 0.17135234354032049604e-2_f64 * t25944 * t25946;
    let t25949 = t25920 * t1426;
    let t25950 = t7063 * t25949;
    let t25951 = t25950 * t7286;
    let t25953 = t7285 * t2470;
    let t25955 = 0.17135234354032049604e-1_f64 * t7289 * t25953;
    let t25969 = t7259 * t3974;
    (t25946, t25948, t25949, t25951, t25953, t25955, t25969)
}
