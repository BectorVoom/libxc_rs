//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 875/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk875<F: Float>(t1495: F, t5880: F, t1468: F, t1464: F, t1943: F, t3717: F, t1385: F, t1459: F, t2006: F, t303: F, t1364: F, t1387: F, t1944: F, t3961: F, t3964: F, t4115: F, t4117: F, t5742: F, t5750: F, t5754: F, t5759: F, t5762: F, t5764: F, t5766: F, t5771: F, t5774: F, t5777: F, t5783: F, t5873: F, t5878: F) -> (F, F, F, F, F, F, F, F) {
    let t5881 = t1495 * t5880;
    let t5882 = t1468 * t5881;
    let t5883 = t1464 * t5882;
    let t5885 = t1943 * t3717;
    let t5886 = t5885 * t1385;
    let t5891 = t1459 * t2006;
    let t5892 = t303 * t5891;
    let t5894 = -F::cast_from(0.66725e-1_f64) * t5742 * t1387 - F::cast_from(0.66725e-1_f64) * t3964 * t1944 - F::cast_from(0.24872916666666666666e-2_f64) * t5750 + F::cast_from(0.16581944444444444444e-2_f64) * t5754 - F::cast_from(0.44218518518518518517e-2_f64) * t5759 + F::cast_from(0.16581944444444444444e-2_f64) * t5762 + F::cast_from(0.11054629629629629629e-2_f64) * t5764 - F::cast_from(0.16581944444444444444e-2_f64) * t5766 - t4115 + F::cast_from(0.16581944444444444444e-2_f64) * t4117 - F::cast_from(0.44218518518518518517e-2_f64) * t5771 + F::cast_from(0.16581944444444444444e-2_f64) * t5774 + F::cast_from(0.16581944444444444444e-2_f64) * t5777 - F::cast_from(0.33163888888888888888e-2_f64) * t5783 + F::cast_from(0.24872916666666666666e-2_f64) * t5873 - F::cast_from(0.55273148148148148147e-3_f64) * t5878 - F::cast_from(0.16581944444444444444e-2_f64) * t5883 + F::cast_from(0.66725e-1_f64) * t1364 * t5886 + F::cast_from(0.890445125e-2_f64) * t3961 * t5886 - F::cast_from(0.66327777777777777776e-2_f64) * t5892;
    (t5881, t5882, t5883, t5885, t5886, t5891, t5892, t5894)
}
