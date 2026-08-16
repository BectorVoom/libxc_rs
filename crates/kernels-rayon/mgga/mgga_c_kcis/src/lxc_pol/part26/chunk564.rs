//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 564/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk564(t1385: f64, t5885: f64, t1459: f64, t2006: f64, t303: f64, t1364: f64, t1387: f64, t1944: f64, t3961: f64, t3964: f64, t4115: f64, t4117: f64, t5742: f64, t5750: f64, t5754: f64, t5759: f64, t5762: f64, t5764: f64, t5766: f64, t5771: f64, t5774: f64, t5777: f64, t5783: f64, t5873: f64, t5878: f64, t5883: f64) -> (f64, f64, f64, f64) {
    let t5886 = t5885 * t1385;
    let t5891 = t1459 * t2006;
    let t5892 = t303 * t5891;
    let t5894 = -0.66725e-1_f64 * t5742 * t1387 - 0.66725e-1_f64 * t3964 * t1944 - 0.24872916666666666666e-2_f64 * t5750 + 0.16581944444444444444e-2_f64 * t5754 - 0.44218518518518518517e-2_f64 * t5759 + 0.16581944444444444444e-2_f64 * t5762 + 0.11054629629629629629e-2_f64 * t5764 - 0.16581944444444444444e-2_f64 * t5766 - t4115 + 0.16581944444444444444e-2_f64 * t4117 - 0.44218518518518518517e-2_f64 * t5771 + 0.16581944444444444444e-2_f64 * t5774 + 0.16581944444444444444e-2_f64 * t5777 - 0.33163888888888888888e-2_f64 * t5783 + 0.24872916666666666666e-2_f64 * t5873 - 0.55273148148148148147e-3_f64 * t5878 - 0.16581944444444444444e-2_f64 * t5883 + 0.66725e-1_f64 * t1364 * t5886 + 0.890445125e-2_f64 * t3961 * t5886 - 0.66327777777777777776e-2_f64 * t5892;
    (t5886, t5891, t5892, t5894)
}
