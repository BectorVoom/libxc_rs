//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1258/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1258(t21933: f64, t21935: f64, t21951: f64, t26513: f64, t26527: f64, t26535: f64, t26585: f64, t26588: f64, t26592: f64, t2945: f64, t301: f64, t30790: f64, t757: f64, t758: f64, t761: f64, t7796: f64, t9194: f64) -> f64 {
    let t30803 = 0.42874018118069736972e-3_f64 * t26513 - 0.1543464652250510531e-1_f64 * t2945 * t758 * t7796 * t9194 + 0.21437009059034868486e-3_f64 * t757 * t758 * t301 * t30790 * t761 + 0.19055119163586549765e-3_f64 * t21933 - 0.45732285992607719437e-2_f64 * t26527 + 0.76220476654346199061e-3_f64 * t21935 - 0.14291339372689912324e-3_f64 * t26535 + t26585 / 48.0_f64 - t26588 / 16.0_f64 + t26592 / 24.0_f64 + t21951;
    t30803
}
