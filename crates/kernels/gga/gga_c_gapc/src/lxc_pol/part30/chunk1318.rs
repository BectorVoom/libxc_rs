//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1318/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1318<F: Float>(t34742: F, t34745: F, t34747: F, t34749: F, t34752: F, t34755: F, t34757: F, t34759: F, t34761: F, t34764: F, t34767: F, t34772: F, t34776: F, t34779: F, t34782: F, t34785: F, t34788: F, t34791: F, t34794: F, t34797: F, t34802: F, t34804: F) -> (F, F) {
    let t38269 = -F::cast_from(0.19666550313313802087e-6_f64) * t34742 - F::cast_from(0.27012148473991046866e-5_f64) * t34745 - F::cast_from(0.2023819338830593704e-6_f64) * t34747 - F::cast_from(0.30917817036057089854e-5_f64) * t34749 + F::cast_from(0.16193229166666666668e-3_f64) * t34752 + F::cast_from(0.8096614583333333334e-3_f64) * t34755 - F::cast_from(0.9275345110817126956e-4_f64) * t34757 + F::cast_from(0.13130905508190092226e-7_f64) * t34759 - F::cast_from(0.13937148427849636339e-3_f64) * t34761 - F::cast_from(0.12670134934408760308e-4_f64) * t34764 - F::cast_from(0.12670134934408760308e-4_f64) * t34767;
    let t38282 = F::cast_from(0.38673709012042260328e-7_f64) * t34772 - F::cast_from(0.10477996894995983065e-8_f64) * t34776 - F::cast_from(0.27826035332451380868e-3_f64) * t34779 - F::cast_from(0.40083661544871514617e-6_f64) * t34782 - F::cast_from(0.13900948042322754167e-2_f64) * t34785 + F::cast_from(0.10016653645505750616e-4_f64) * t34788 + F::cast_from(0.86880925264517213544e-4_f64) * t34791 + F::cast_from(0.22745373045674261828e-4_f64) * t34794 + F::cast_from(0.58931366113588206214e-8_f64) * t34797 - F::cast_from(0.19666550313313802087e-7_f64) * t34802 + F::cast_from(0.18550690221634253912e-3_f64) * t34804;
    (t38269, t38282)
}
