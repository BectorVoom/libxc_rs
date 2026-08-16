//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1436/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1436(t34092: f64, t34100: f64, t36812: f64, t36813: f64, t36814: f64, t36815: f64, t36816: f64, t36817: f64, t36818: f64, t36820: f64, t36821: f64, t34125: f64, t34135: f64, t36824: f64, t36825: f64, t36826: f64, t36827: f64, t36828: f64, t36829: f64, t36830: f64, t36832: f64, t36833: f64) -> (f64, f64) {
    let t38792 = -t36812 - t36813 - t36814 - t36815 - t36816 + t36817 + t36818 - 0.98380106748709416171e-8_f64 * t34092 - t36820 + t36821 - 0.36231816839129402172e-6_f64 * t34100;
    let t38795 = t36824 + t36825 + t36826 - t36827 + t36828 - t36829 - t36830 + 0.95956020918421216159e-7_f64 * t34125 + t36832 - t36833 + 0.25301106770833333334e-5_f64 * t34135;
    (t38792, t38795)
}
