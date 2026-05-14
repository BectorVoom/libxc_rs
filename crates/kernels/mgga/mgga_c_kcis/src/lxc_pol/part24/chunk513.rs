//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 513/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk513<F: Float>(t3218: F, t4813: F, t1021: F, t1092: F, t1121: F, t1767: F, t1022: F, t3227: F, t1133: F, t1131: F, t1096: F, t2823: F, t2836: F, t2862: F, t3052: F, t3174: F, t4550: F, t4558: F, t4775: F, t4779: F, t4782: F, t4787: F, t4790: F, t4794: F, t4798: F, t4803: F, t4808: F, t979: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4814 = t3218 * t4813;
    let t4815 = t1021 * t4814;
    let t4816 = t1092 * t4815;
    let t4818 = t1767 * t1121;
    let t4819 = t1022 * t4818;
    let t4820 = t3227 * t4819;
    let t4821 = t1092 * t4820;
    let t4823 = t1767 * t1133;
    let t4824 = t1131 * t4823;
    let t4825 = t1096 * t4824;
    let t4826 = t1092 * t4825;
    let t4828 = -0.33163888888888888888e-2 * t4550 + 0.27636574074074074073e-2 * t4558 - 0.24872916666666666666e-2 * t4775 + 0.11054629629629629629e-2 * t2823 - 0.24872916666666666666e-2 * t4779 + 0.66725e-1 * t979 * t4782 + 0.890445125e-2 * t2836 * t4782 + 0.16581944444444444444e-2 * t4787 - 0.66327777777777777776e-2 * t4790 - 0.16581944444444444444e-2 * t4794 + 0.11054629629629629629e-2 * t4798 - 0.16581944444444444444e-2 * t4803 + 0.11054629629629629629e-2 * t4808 - 0.16581944444444444444e-2 * t2862 + 0.16581944444444444444e-2 * t3052 + 0.11054629629629629629e-2 * t3174 - 0.55273148148148148147e-3 * t4816 + 0.49745833333333333332e-2 * t4821 - 0.16581944444444444444e-2 * t4826;
    (t4814, t4815, t4816, t4818, t4819, t4820, t4821, t4823, t4824, t4825, t4826, t4828)
}
