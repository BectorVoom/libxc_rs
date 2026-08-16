//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1070/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1070(t4773: f64, t570: f64, t30661: f64, t30664: f64, t30670: f64, t30672: f64, t30675: f64, t30690: f64, t30695: f64, t30705: f64, t30709: f64, t34655: f64, t34657: f64, t34660: f64, t34663: f64, t34667: f64, t34671: f64, t34675: f64, t34684: f64) -> f64 {
    let t34686 = t570 * t4773;
    let t34688 = 0.40015750243531754508e-2_f64 * t30661 - t30664 - t30670 + t30672 - t34655 - 0.17149607247227894789e-2_f64 * t30675 - t34657 / 96.0_f64 + t34660 + 0.31448092289604152068e-3_f64 * t34663 + 0.64311027177104605458e-3_f64 * t34667 + 0.47172138434406228102e-2_f64 * t34671 + 0.41930789719472202758e-3_f64 * t34675 - 0.34299214494455789578e-2_f64 * t30690 + 0.7145669686344956162e-3_f64 * t30695 - 0.10482697429868050689e-2_f64 * t30705 - 0.62896184579208304134e-3_f64 * t30709 - 0.64311027177104605458e-3_f64 * t34684 - t34686 / 48.0_f64;
    t34688
}
