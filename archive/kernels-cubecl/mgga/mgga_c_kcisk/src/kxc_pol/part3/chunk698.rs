//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 698/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk698<F: Float>(t10880: F, t1773: F, t4998: F, t5025: F, t25: F, t5005: F, t5008: F, t1744: F, t4928: F, t1746: F, t4948: F, t4954: F, t7181: F) -> (F, F, F, F, F, F) {
    let t10881 = t1773 * t10880;
    let t10883 = t4998 * t5025;
    let t10884 = t1773 * t10883;
    let t10886 = t25 * t5005;
    let t10887 = t10886 * t5008;
    let t10888 = t1773 * t10887;
    let t10892 = t4928 * t1744;
    let t10893 = t1746 * t4948;
    let t10894 = t10892 * t10893;
    let t10898 = t4954 * t4948 * t7181;
    (t10881, t10884, t10886, t10888, t10894, t10898)
}
