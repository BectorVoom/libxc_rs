//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 295/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk295(t880: f64, t906: f64, t886: f64, t897: f64, t902: f64, t910: f64) -> (f64, f64, f64) {
    let t926 = 0.516475e0_f64 * t880;
    let t929 = 0.104195e0_f64 * t906;
    let t931 = 0.3529725e1_f64 * t897 - t926 - 0.516475e0_f64 * t886 + 0.6311625e0_f64 * t902 - t929 - 0.104195e0_f64 * t910;
    (t926, t929, t931)
}
