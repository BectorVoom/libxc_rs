//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 599/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk599<F: Float>(t143: F, t1849: F, t3290: F, t682: F, t1060: F, t1814: F, t1824: F, t3293: F, t681: F, t4658: F, t4684: F, t1835: F, t4644: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5089 = t143 * t1849;
    let t5090 = t682 * t3290;
    let t5093 = t1814 * t1060;
    let t5094 = t5093 * t1824;
    let t5097 = t682 * t3293;
    let t5100 = t681 * t681;
    let t5101 = F::cast_from(1.0_f64) / t5100;
    let t5102 = t5101 * t4658;
    let t5105 = t1814 * t4684;
    let t5111 = t1835 * t4644;
    (t5089, t5090, t5093, t5094, t5097, t5100, t5101, t5102, t5105, t5111)
}
