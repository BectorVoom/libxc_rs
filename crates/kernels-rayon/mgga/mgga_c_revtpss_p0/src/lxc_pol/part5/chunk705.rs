//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 705/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk705(t2908: f64, t4574: f64, t141: f64, t4579: f64, t930: f64, t4583: f64, t2848: f64, t2892: f64, t2905: f64, t2906: f64, t4571: f64, t4576: f64, t4581: f64, t4585: f64, t4599: f64, t4607: f64, t4615: f64, t4617: f64, t4620: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4622 = t2908 * t4574;
    let t4623 = t141 * t4622;
    let t4625 = t930 * t4579;
    let t4626 = t141 * t4625;
    let t4628 = t930 * t4583;
    let t4629 = t141 * t4628;
    let t4631 = -0.9494625e0_f64 * t4599 + 0.1898925e1_f64 * t4607 + t2892 + 0.99655555555555555557e-1_f64 * t2848 + 0.99655555555555555557e-1_f64 * t4571 - 0.19931111111111111111e0_f64 * t4576 + 0.59793333333333333334e0_f64 * t4581 - 0.29896666666666666667e0_f64 * t4585 + 0.15358125e0_f64 * t4615 + 0.3071625e0_f64 * t4617 + t2905 + 0.54771111111111111111e-1_f64 * t2906 + 0.54771111111111111111e-1_f64 * t4620 - 0.27385555555555555556e-1_f64 * t4623 + 0.16431333333333333333e0_f64 * t4626 - 0.82156666666666666667e-1_f64 * t4629;
    (t4622, t4623, t4625, t4626, t4628, t4629, t4631)
}
