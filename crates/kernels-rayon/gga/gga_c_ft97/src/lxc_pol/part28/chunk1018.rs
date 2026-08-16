//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1018/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1018(t144822: f64, t25928: f64, t5674: f64, t144769: f64, t144773: f64, t144777: f64, t144781: f64, t144786: f64, t144790: f64, t144794: f64, t144798: f64, t144803: f64, t144805: f64, t144807: f64, t144811: f64, t144815: f64, t144817: f64, t144820: f64) -> (f64, f64) {
    let t144824 = t5674 * t25928 * t144822;
    let t144826 = -t144769 / 2.0_f64 - 3.0_f64 * t144773 + 3.0_f64 * t144777 + 8.0_f64 * t144781 - 15.0_f64 / 4.0_f64 * t144786 + t144790 / 3.0_f64 - 4.0_f64 / 3.0_f64 * t144794 + t144798 / 6.0_f64 + 4.0_f64 / 3.0_f64 * t144803 - 4.0_f64 / 9.0_f64 * t144805 - t144807 / 18.0_f64 - t144811 / 3.0_f64 + t144815 - 2.0_f64 / 3.0_f64 * t144817 + 2.0_f64 / 3.0_f64 * t144820 + t144824 / 9.0_f64;
    (t144824, t144826)
}
