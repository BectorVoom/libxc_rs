//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 628/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk628(t1224: f64, t1697: f64, t8518: f64, t4835: f64, t7076: f64, t8684: f64, t8687: f64, t2417: f64, t1725: f64, t2408: f64, t4864: f64, t4868: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8690 = t1224 * t1697 * t8518;
    let t8692 = t4835 + 0.11872222222222222222e-1_f64 * t7076 - 0.11872222222222222222e-1_f64 * t8684 + 0.35616666666666666666e-1_f64 * t8687 - 0.17808333333333333333e-1_f64 * t8690;
    let t8697 = t2417 * t2417;
    let t8698 = t8697 * t1725;
    let t8701 = t2408 * t2408;
    let t8702 = t4864 * t8701;
    let t8708 = t4868 + 2.0_f64 / 9.0_f64 * t7076 - 2.0_f64 / 9.0_f64 * t8684 + 2.0_f64 / 3.0_f64 * t8687 - t8690 / 3.0_f64;
    (t8690, t8692, t8697, t8698, t8701, t8702, t8708)
}
