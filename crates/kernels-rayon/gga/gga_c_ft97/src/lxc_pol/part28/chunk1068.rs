//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1068/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1068(t1286: f64, t34353: f64, t376: f64, t1852: f64, t34511: f64, t492: f64, t144769: f64, t144773: f64, t144777: f64, t144781: f64, t144786: f64, t144790: f64, t144794: f64, t144798: f64, t144803: f64, t144805: f64, t144807: f64, t144811: f64, t144815: f64, t144817: f64, t144820: f64, t144824: f64) -> (f64, f64, f64) {
    let t145771 = t1286 * t376 * t34353;
    let t145774 = t1852 * t34511 * t492;
    let t145790 = -t144769 / 6.0_f64 - t144773 + t144777 + 8.0_f64 / 3.0_f64 * t144781 - 5.0_f64 / 4.0_f64 * t144786 + t144790 / 9.0_f64 - 4.0_f64 / 9.0_f64 * t144794 + t144798 / 18.0_f64 + 4.0_f64 / 9.0_f64 * t144803 - 4.0_f64 / 27.0_f64 * t144805 - t144807 / 54.0_f64 - t144811 / 9.0_f64 + t144815 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t144817 + 2.0_f64 / 9.0_f64 * t144820 + t144824 / 27.0_f64;
    (t145771, t145774, t145790)
}
