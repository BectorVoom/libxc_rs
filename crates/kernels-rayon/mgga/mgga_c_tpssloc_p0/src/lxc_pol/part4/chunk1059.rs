//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1059/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1059(t17375: f64, t17449: f64, t17516: f64, t17558: f64, t300: f64, t2940: f64, t5808: f64, t10629: f64, t5774: f64, t10632: f64, t950: f64, t959: f64) -> (f64, f64, f64) {
    let t17561 = t300 * (t17375 + t17449 + t17516 + t17558);
    let t17563 = 0.5848223622634646207e0_f64 * t2940 * t5808;
    let t17564 = t10629 * t5774;
    let t17565 = t10632 * t950;
    let t17566 = t17564 * t17565;
    let t17568 = 0.10254018858216406658e4_f64 * t959 * t17566;
    (t17561, t17563, t17568)
}
