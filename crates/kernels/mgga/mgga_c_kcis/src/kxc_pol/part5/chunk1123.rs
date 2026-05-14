//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1123/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1123<F: Float>(t15887: F, t20994: F, t4160: F, t1889: F, t5880: F, t12281: F, t2001: F, t833: F, t5440: F, t15878: F, t5426: F, t5661: F, t12286: F, t15844: F, t20970: F, t20977: F, t20982: F, t20987: F, t20991: F, t7043: F) -> (F, F, F, F, F) {
    let t20995 = t15887 * t20994;
    let t20996 = t4160 * t20995;
    let t20998 = t1889 * t5880;
    let t20999 = t12281 * t20998;
    let t21000 = t4160 * t20999;
    let t21002 = t2001 * t833;
    let t21003 = t5440 * t21002;
    let t21004 = t15878 * t21003;
    let t21005 = t4160 * t21004;
    let t21007 = t5426 * t21002;
    let t21008 = t15878 * t21007;
    let t21009 = t5661 * t21008;
    let t21011 = -0.11054629629629629629e-2 * t20970 + 0.890445125e-2 * t12286 * t7043 + 0.33163888888888888888e-2 * t20977 - 0.22109259259259259259e-2 * t20982 + 0.99491666666666666664e-2 * t20987 + 0.13265555555555555555e-1 * t20991 + 0.22109259259259259259e-2 * t15844 + 0.66327777777777777776e-2 * t20996 - 0.22109259259259259259e-2 * t21000 + 0.66327777777777777776e-2 * t21005 - 0.55273148148148148147e-2 * t21009;
    (t20996, t21000, t21005, t21009, t21011)
}
