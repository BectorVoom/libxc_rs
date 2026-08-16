//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1081/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1081(t1882: f64, t34629: f64, t102921: f64, t5731: f64, t34732: f64, t34718: f64, t34722: f64, t22943: f64, t25590: f64, t34726: f64, t8392: f64, t102776: f64, t103108: f64, t103823: f64, t11490: f64, t117775: f64, t11810: f64, t11906: f64, t1307: f64, t137826: f64, t137836: f64, t1901: f64, t23339: f64, t26042: f64, t26145: f64, t26167: f64, t32516: f64, t3291: f64, t34670: f64, t446: f64, t452: f64, t47007: f64, t5718: f64, t5722: f64, t6465: f64, t6538: f64, t7211: f64, t83: f64, t91583: f64, t92049: f64) -> (f64, f64, f64) {
    let t146206 = t1882 * t34629;
    let t146208 = t102921 * t5731;
    let t146212 = t1882 * t34732;
    let t146214 = t1882 * t34718;
    let t146216 = t1882 * t34722;
    let t146218 = t22943 * t25590;
    let t146237 = t8392 * t34726;
    let t146263 = 2.0_f64 / 9.0_f64 * t137826 - t146206 / 9.0_f64 + 4.0_f64 / 3.0_f64 * t446 * t83 * t146208 + 2.0_f64 / 9.0_f64 * t146212 - 2.0_f64 / 9.0_f64 * t146214 + 2.0_f64 / 9.0_f64 * t146216 - t137836 + 4.0_f64 / 3.0_f64 * t446 * t83 * t146218 - t446 * t452 * t3291 * t7211 / 3.0_f64 - 4.0_f64 / 3.0_f64 * t1901 * t11810 * t103108 * t5722 - 4.0_f64 / 3.0_f64 * t1901 * t11490 * t117775 * t5731 + 2.0_f64 / 9.0_f64 * t1901 * t103823 * t5718 + 4.0_f64 / 9.0_f64 * t146237 + 2.0_f64 / 9.0_f64 * t1901 * t92049 * t6465 - 4.0_f64 / 3.0_f64 * t1901 * t102776 * t26167 - 4.0_f64 / 3.0_f64 * t1901 * t47007 * t34670 - 4.0_f64 / 3.0_f64 * t1901 * t11810 * t91583 * t6538 - 4.0_f64 / 3.0_f64 * t1901 * t11810 * t23339 * t26145 + t1901 * t11906 * t32516 / 9.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t452 * t26042 * t1307;
    (t146208, t146218, t146263)
}
