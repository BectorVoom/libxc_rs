//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 883/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk883<F: Float>(t17057: F, t8673: F, t1869: F, t2527: F, t8939: F, t1801: F, t5062: F, t1693: F, t23874: F, t23876: F, t23878: F, t23880: F, t23894: F, t23922: F, t2470: F, t28262: F, t28329: F, t4823: F, t7278: F, t8846: F, t8852: F) -> (F, F, F, F) {
    let t28731 = t17057 * t8673;
    let t28732 = t1869 * t28731;
    let t28749 = t8939 * t2527;
    let t28750 = t1801 * t28749;
    let t28751 = t5062 * t28750;
    let t28752 = t1869 * t28751;
    let t28754 = F::new(0.1492375e-1) * t28732 - F::new(0.386e0) * t1693 * t28262 - F::new(0.579e0) * t23922 * t2470 + F::cast_from(0.33163888888888888887e-2_f64) * t23874 - F::cast_from(0.99491666666666666664e-2_f64) * t23876 - F::cast_from(0.11054629629629629629e-2_f64) * t23878 - F::cast_from(0.17687407407407407407e-1_f64) * t23880 - F::new(0.579e0) * t7278 * t8846 + F::new(0.223494e0) * t4823 * t28329 + F::new(0.579e0) * t7278 * t8852 - F::cast_from(0.49745833333333333332e-2_f64) * t23894 + F::new(0.1492375e-1) * t28752;
    (t28732, t28749, t28752, t28754)
}
