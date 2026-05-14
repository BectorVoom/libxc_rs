//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1236/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1236<F: Float>(t7278: F, t9931: F, t23922: F, t2781: F, t717: F, t8878: F, t415: F, t8882: F, t9926: F, t8518: F, t9665: F, t1775: F, t8510: F, t5006: F, t2785: F, t32897: F, t34065: F, t34067: F, t34073: F, t34081: F, t34122: F, t34125: F, t35097: F, t35133: F, t35136: F, t9649: F, t9664: F, t9936: F, t9940: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t35143 = t7278 * t9931;
    let t35146 = t23922 * t2781;
    let t35149 = t717 * t8878;
    let t35150 = t415 * t35149;
    let t35152 = t717 * t8882;
    let t35153 = t415 * t35152;
    let t35155 = t7278 * t9926;
    let t35158 = t9665 * t8518;
    let t35159 = t1775 * t35158;
    let t35162 = t9665 * t8510;
    let t35163 = t5006 * t35162;
    let t35173 = -t32897 + 0.27636574074074074073e-2 * t35133 + 0.40208333333333333335e-2 * t9649 * t35136 + 0.20833333333333333334e-1 * t34073 * t9940 + 0.22109259259259259258e-2 * t34065 + 0.69444444444444444446e-2 * t34067 + 0.55555555555555555558e-1 * t35143 * t2785 - 0.10416666666666666667e-1 * t35146 * t2785 - 0.88437037037037037034e-2 * t35150 + 0.16581944444444444444e-2 * t35153 - 0.20833333333333333334e-1 * t35155 * t2785 - 0.34722222222222222223e-2 * t9664 * t35159 - 0.46296296296296296297e-2 * t9664 * t35163 - 0.69444444444444444446e-2 * t34122 * t9936 + 0.18518518518518518519e-1 * t34125 * t9936 - 0.8041666666666666667e-2 * t9649 * t35097 - 0.33163888888888888888e-2 * t34081;
    (t35143, t35146, t35149, t35150, t35152, t35153, t35155, t35158, t35159, t35162, t35163, t35173)
}
