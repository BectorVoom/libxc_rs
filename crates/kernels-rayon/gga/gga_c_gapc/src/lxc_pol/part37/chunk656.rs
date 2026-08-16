//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 656/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk656(t209: f64, t3903: f64, t3692: f64, t3704: f64, t3715: f64, t3719: f64, t3811: f64, t3812: f64, t3813: f64, t3814: f64, t3815: f64, t3816: f64, t3819: f64) -> (f64, f64) {
    let t3904 = t3903 * t209;
    let t3909 = t3811 - t3812 - t3813 + t3814 - t3815 - t3816 + 0.57970906942607043475e-5_f64 * t3692 - 0.49166375783284505216e-8_f64 * t3704 + t3819 + 0.6629778687778673199e-7_f64 * t3715 - 0.90579542097823505428e-7_f64 * t3719;
    (t3904, t3909)
}
