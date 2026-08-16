//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 981/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk981(t135: f64, t7189: f64, t136678: f64, t23745: f64, t136457: f64, t32809: f64, t32795: f64, t32796: f64, t549: f64, t1691: f64, t23742: f64, t138838: f64, t23842: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t138969 = t7189 * t135;
    let t138991 = t23745 * t136678;
    let t138996 = t32809 * t136457;
    let t139009 = t32795 * t32796 * t549;
    let t139046 = t549 * t1691;
    let t139057 = t23742 * t136678;
    let t139065 = t23842 * t138838;
    (t138969, t138991, t138996, t139009, t139046, t139057, t139065)
}
