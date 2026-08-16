//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 787/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk787(t1131: f64, t4823: f64, t1096: f64, t1092: f64, t2823: f64, t2836: f64, t2862: f64, t3052: f64, t3174: f64, t4550: f64, t4558: f64, t4775: f64, t4779: f64, t4782: f64, t4787: f64, t4790: f64, t4794: f64, t4798: f64, t4803: f64, t4808: f64, t4816: f64, t4821: f64, t979: f64) -> (f64, f64, f64, f64) {
    let t4824 = t1131 * t4823;
    let t4825 = t1096 * t4824;
    let t4826 = t1092 * t4825;
    let t4828 = -0.33163888888888888888e-2_f64 * t4550 + 0.27636574074074074073e-2_f64 * t4558 - 0.24872916666666666666e-2_f64 * t4775 + 0.11054629629629629629e-2_f64 * t2823 - 0.24872916666666666666e-2_f64 * t4779 + 0.66725e-1_f64 * t979 * t4782 + 0.890445125e-2_f64 * t2836 * t4782 + 0.16581944444444444444e-2_f64 * t4787 - 0.66327777777777777776e-2_f64 * t4790 - 0.16581944444444444444e-2_f64 * t4794 + 0.11054629629629629629e-2_f64 * t4798 - 0.16581944444444444444e-2_f64 * t4803 + 0.11054629629629629629e-2_f64 * t4808 - 0.16581944444444444444e-2_f64 * t2862 + 0.16581944444444444444e-2_f64 * t3052 + 0.11054629629629629629e-2_f64 * t3174 - 0.55273148148148148147e-3_f64 * t4816 + 0.49745833333333333332e-2_f64 * t4821 - 0.16581944444444444444e-2_f64 * t4826;
    (t4824, t4825, t4826, t4828)
}
