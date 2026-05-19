//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 612/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk612<F: Float>(t1131: F, t4823: F, t1096: F, t1092: F, t2823: F, t2836: F, t2862: F, t3052: F, t3174: F, t4550: F, t4558: F, t4775: F, t4779: F, t4782: F, t4787: F, t4790: F, t4794: F, t4798: F, t4803: F, t4808: F, t4816: F, t4821: F, t979: F) -> (F, F, F, F) {
    let t4824 = t1131 * t4823;
    let t4825 = t1096 * t4824;
    let t4826 = t1092 * t4825;
    let t4828 = -F::cast_from(0.33163888888888888888e-2_f64) * t4550 + F::cast_from(0.27636574074074074073e-2_f64) * t4558 - F::cast_from(0.24872916666666666666e-2_f64) * t4775 + F::cast_from(0.11054629629629629629e-2_f64) * t2823 - F::cast_from(0.24872916666666666666e-2_f64) * t4779 + F::new(0.66725e-1) * t979 * t4782 + F::cast_from(0.890445125e-2_f64) * t2836 * t4782 + F::cast_from(0.16581944444444444444e-2_f64) * t4787 - F::cast_from(0.66327777777777777776e-2_f64) * t4790 - F::cast_from(0.16581944444444444444e-2_f64) * t4794 + F::cast_from(0.11054629629629629629e-2_f64) * t4798 - F::cast_from(0.16581944444444444444e-2_f64) * t4803 + F::cast_from(0.11054629629629629629e-2_f64) * t4808 - F::cast_from(0.16581944444444444444e-2_f64) * t2862 + F::cast_from(0.16581944444444444444e-2_f64) * t3052 + F::cast_from(0.11054629629629629629e-2_f64) * t3174 - F::cast_from(0.55273148148148148147e-3_f64) * t4816 + F::cast_from(0.49745833333333333332e-2_f64) * t4821 - F::cast_from(0.16581944444444444444e-2_f64) * t4826;
    (t4824, t4825, t4826, t4828)
}
