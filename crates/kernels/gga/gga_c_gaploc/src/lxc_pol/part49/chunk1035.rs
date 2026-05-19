//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1035/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1035<F: Float>(t1457: F, t2103: F, t43217: F, t43001: F, t10867: F, t9989: F, t10086: F, t10811: F, t43508: F, t7427: F, t7573: F, t326: F, t43486: F, t825: F) -> (F, F, F, F, F, F) {
    let t43729 = F::cast_from(0.71500979903700853338e0_f64) * t2103 * t1457 * t43217;
    let t43731 = t2103 * t1457 * t43001;
    let t43735 = F::cast_from(0.25025342966295298669e1_f64) * t10867 * t1457 * t9989;
    let t43737 = F::cast_from(0.42900587942220512003e1_f64) * t10811 * t10086;
    let t43740 = F::cast_from(0.62115540045351614476e2_f64) * t7427 * t7573 * t43508;
    let t43743 = F::cast_from(0.18404604457881959845e2_f64) * t825 * t326 * t43486;
    (t43729, t43731, t43735, t43737, t43740, t43743)
}
