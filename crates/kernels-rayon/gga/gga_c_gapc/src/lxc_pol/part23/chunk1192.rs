//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 1192/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk1192(t20171: f64, t33287: f64, t5708: f64, t19533: f64, t19535: f64, t34742: f64, t34745: f64, t34747: f64, t34749: f64, t34752: f64, t34755: f64, t34757: f64, t34759: f64, t34761: f64) -> f64 {
    let t34764 = t5708 * t33287 * t20171;
    let t34767 = t19533 * t33287 * t19535;
    let t34769 = -0.49166375783284505216e-7_f64 * t34742 - 0.67530371184977617164e-6_f64 * t34745 - 0.50595483470764842601e-7_f64 * t34747 - 0.77294542590142724635e-6_f64 * t34749 + 0.40483072916666666668e-4_f64 * t34752 + 0.20241536458333333334e-3_f64 * t34755 - 0.2318836277704281739e-4_f64 * t34757 + 0.32827263770475230566e-8_f64 * t34759 - 0.34842871069624090849e-4_f64 * t34761 - 0.31675337336021900772e-5_f64 * t34764 - 0.31675337336021900772e-5_f64 * t34767;
    t34769
}
