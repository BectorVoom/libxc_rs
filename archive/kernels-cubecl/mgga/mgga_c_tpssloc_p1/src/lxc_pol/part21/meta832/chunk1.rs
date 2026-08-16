//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2933/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2933<F: Float>(t10236: F, t17686: F, t43070: F, t10254: F, t17635: F, t12652: F, t13554: F, t10263: F, t13835: F, t1539: F, t17844: F, t23547: F, t2960: F, t2986: F, t2988: F, t43069: F, t4510: F, t4518: F, t4531: F, t47907: F, t5845: F, t61065: F, t61066: F, t61074: F, t61078: F, t984: F) -> (F, F) {
    let t61082 = t10236 * t17686;
    let t61086 = t43070 * t17686;
    let t61094 = t10254 * t17635;
    let t61098 = t13554 * t12652;
    let t61102 = F::cast_from(0.22222222222222222222e-2_f64) * t61065 * t61066 * t984 * t13835 - F::cast_from(0.37037037037037037036e-3_f64) * t47907 - F::cast_from(0.81481481481481481481e-2_f64) * t10263 * t5845 + F::cast_from(0.14814814814814814814e-2_f64) * t61074 + F::cast_from(0.44444444444444444444e-2_f64) * t2960 * t17844 + F::cast_from(0.66666666666666666665e-2_f64) * t2986 * t4518 * t61078 - F::cast_from(0.33333333333333333332e-2_f64) * t2986 * t2988 * t61082 - F::cast_from(0.17283950617283950617e-2_f64) * t2986 * t43069 * t61086 - F::cast_from(0.55555555555555555554e-3_f64) * t2986 * t4531 * t23547 * t1539 + F::cast_from(0.11111111111111111111e-2_f64) * t2986 * t2988 * t61094 - F::cast_from(0.88888888888888888886e-2_f64) * t2986 * t4510 * t61098;
    (t61098, t61102)
}
