//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1195/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1195(t11748: f64, t19210: f64, t2597: f64, t11397: f64, t11980: f64, t761: f64, t11979: f64, t3074: f64, t33966: f64, t3775: f64, t9538: f64, t33258: f64, t3698: f64, t3780: f64) -> (f64, f64, f64, f64, f64) {
    let t33972 = t11748 * t2597 * t19210;
    let t33975 = t761 * t11397 * t11980;
    let t33977 = t3074 * t11979;
    let t33978 = t33966 * t33977;
    let t33980 = t3775 * t9538;
    let t33983 = t33258 * t3698 * t3780;
    (t33972, t33975, t33978, t33980, t33983)
}
