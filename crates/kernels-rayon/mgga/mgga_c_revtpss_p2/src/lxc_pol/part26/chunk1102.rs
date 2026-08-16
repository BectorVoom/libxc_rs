//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1102/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1102(t30: f64, t892: f64, t14685: f64, t1941: f64, t241: f64, t25260: f64, t820: f64, t1955: f64, t7057: f64, t11064: f64, t33: f64, t7283: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27159 = t892 * t30;
    let t27221 = t1941 * t14685;
    let t27261 = t820 * t25260 * t241;
    let t27353 = t1955 * t7057;
    let t27383 = t11064 * t30;
    let t27763 = t892 * t33;
    let t27799 = t11064 * t33;
    let t27868 = t1955 * t7283;
    (t27159, t27221, t27261, t27353, t27383, t27763, t27799, t27868)
}
