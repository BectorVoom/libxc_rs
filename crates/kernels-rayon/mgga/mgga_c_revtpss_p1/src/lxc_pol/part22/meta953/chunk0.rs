//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3196/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3196(t17191: f64, t3566: f64, t3781: f64, t5216: f64, t45618: f64, t460: f64, t487: f64, t43350: f64, t44535: f64, t45607: f64, t13045: f64, t1204: f64, t17948: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t59817 = t3566 * t17191;
    let t59854 = t5216 * t3781;
    let t59864 = t460 * t45618 * t487;
    let t59865 = t43350 * t44535;
    let t59871 = t460 * t45607 * t487;
    let t59872 = t43350 * t13045;
    let t59941 = t1204 * t17948;
    (t59817, t59854, t59864, t59865, t59871, t59872, t59941)
}
