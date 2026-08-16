//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2779/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2779(t10744: f64, t14861: f64, t808: f64, t40791: f64, t4442: f64, t14468: f64, t236: f64, t807: f64, t854: f64, t10489: f64, t14586: f64, t14745: f64, t14791: f64, t1548: f64, t2430: f64, t2730: f64, t2745: f64, t2749: f64, t40655: f64, t40822: f64, t40824: f64, t40836: f64, t40838: f64, t40840: f64, t4362: f64, t4457: f64, t51026: f64, t51028: f64, t51042: f64, t51047: f64, t51049: f64, t51055: f64, t775: f64, t800: f64) -> f64 {
    let t51058 = t10744 * t808 * t14861;
    let t51059 = 0.76230004213927992336e-5_f64 * t51058;
    let t51060 = t40791 * t4442;
    let t51061 = 35.0_f64 / 24.0_f64 * t51060;
    let t51070 = t807 * t236 * t854 * t14468;
    let t51072 = -7.0_f64 / 8.0_f64 * t51026 - 7.0_f64 / 16.0_f64 * t51028 + 3.0_f64 / 16.0_f64 * t2730 * t800 * t14745 * t775 + 3.0_f64 / 16.0_f64 * t2730 * t800 * t4457 * t2430 + t2730 * t800 * t1548 * t10489 / 16.0_f64 - 0.45738002528356795401e-4_f64 * t51042 + 0.24009450146119052705e-1_f64 * t40822 - 0.60023625365297631762e-2_f64 * t40824 - 0.6098400337114239387e-4_f64 * t40836 - 0.12004725073059526352e-1_f64 * t51047 + 0.51448821741683684367e-2_f64 * t2745 * t14791 * t51049 * t2749 - 0.54214778996945588151e-4_f64 * t51055 + t51059 + t51061 - 0.51448821741683684367e-2_f64 * t4362 * t14791 * t14586 * t40655 - 35.0_f64 / 72.0_f64 * t40838 + 7.0_f64 / 144.0_f64 * t40840 + 0.85748036236139473944e-4_f64 * t51070;
    t51072
}
