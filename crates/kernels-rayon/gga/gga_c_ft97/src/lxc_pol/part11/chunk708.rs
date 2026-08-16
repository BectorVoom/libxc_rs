//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 708/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk708(t701: f64, t9670: f64, t704: f64, t8608: f64, t420: f64, t9637: f64, t9639: f64, t9642: f64, t9645: f64, t9648: f64, t9655: f64, t9660: f64, t9663: f64, t9668: f64) -> (f64, f64, f64, f64) {
    let t9671 = t701 * t9670;
    let t9673 = t704 * t8608;
    let t9674 = t420 * t9673;
    let t9675 = t701 * t9674;
    let t9677 = t9637 - 0.42562405586419753086e-2_f64 * t9639 + 0.85124811172839506172e-2_f64 * t9642 - 0.12768721675925925926e-1_f64 * t9645 + 0.63843608379629629629e-2_f64 * t9648 + 0.19862455940329218107e-1_f64 * t9655 - 0.51074886703703703704e-1_f64 * t9660 + 0.25537443351851851852e-1_f64 * t9663 + 0.38306165027777777778e-1_f64 * t9668 - 0.38306165027777777778e-1_f64 * t9671 + 0.6384360837962962963e-2_f64 * t9675;
    (t9671, t9673, t9675, t9677)
}
