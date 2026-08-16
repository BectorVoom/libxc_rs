//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1287/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1287(t26685: f64, t95606: f64, t1646: f64, t27819: f64, t3045: f64, t4947: f64, t14546: f64, t26748: f64, t27816: f64, t27822: f64, t2894: f64, t44684: f64, t44743: f64, t7703: f64, t7704: f64, t95621: f64, t95686: f64, t95688: f64, t95691: f64, t95696: f64) -> (f64, f64) {
    let t95698 = 0.20612155671296296296e-4_f64 * t26685 * t95606;
    let t95713 = t4947 * t27819 * t1646 * t3045;
    let t95718 = -t95686 - 0.66327777777777777776e-2_f64 * t95688 + 0.37101880208333333334e-3_f64 * t26685 * t95691 + t95696 + t95698 + 0.46336805555555555556e-3_f64 * t26748 * t27816 + 0.23168402777777777778e-3_f64 * t7703 * t2894 * t7704 * t44743 + 0.92673611111111111112e-3_f64 * t7703 * t14546 * t7704 * t44684 + 0.46336805555555555556e-3_f64 * t26748 * t27822 + 0.23168402777777777778e-3_f64 * t7703 * t95713 - 0.92754700520833333335e-4_f64 * t26685 * t95621;
    (t95713, t95718)
}
