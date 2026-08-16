//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1108/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1108(t1347: f64, t7614: f64, t2001: f64, t5108: f64, t1967: f64, t8502: f64, t4932: f64, t4552: f64, t1998: f64, t5089: f64, t1451: f64, t7605: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t35709 = t7614 * t1347;
    let t35720 = t2001 * t5108;
    let t35722 = t1967 * t8502;
    let t35724 = t2001 * t4932;
    let t35731 = t2001 * t4552;
    let t35733 = t1998 * t5089;
    let t35736 = t7605 * t1451;
    (t35709, t35720, t35722, t35724, t35731, t35733, t35736)
}
