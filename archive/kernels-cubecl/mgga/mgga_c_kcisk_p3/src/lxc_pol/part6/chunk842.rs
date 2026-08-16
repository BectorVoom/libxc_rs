//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 842/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk842<F: Float>(t1799: F, t28300: F, t2527: F, t8780: F, t5203: F, t1873: F, t1869: F, t6719: F, t8882: F, t10447: F, t967: F) -> (F, F, F, F, F) {
    let t28301 = t1799 * t28300;
    let t28303 = t8780 * t2527;
    let t28304 = t5203 * t28303;
    let t28305 = t1873 * t28304;
    let t28306 = t1869 * t28305;
    let t28308 = t6719 * t8882;
    let t28309 = t1869 * t28308;
    let t28312 = F::cast_from(6.0_f64) * t967 + F::cast_from(6.0_f64) * t10447;
    (t28301, t28303, t28306, t28309, t28312)
}
