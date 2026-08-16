//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2933/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2933(t10236: f64, t17686: f64, t43070: f64, t10254: f64, t17635: f64, t12652: f64, t13554: f64, t10263: f64, t13835: f64, t1539: f64, t17844: f64, t23547: f64, t2960: f64, t2986: f64, t2988: f64, t43069: f64, t4510: f64, t4518: f64, t4531: f64, t47907: f64, t5845: f64, t61065: f64, t61066: f64, t61074: f64, t61078: f64, t984: f64) -> (f64, f64) {
    let t61082 = t10236 * t17686;
    let t61086 = t43070 * t17686;
    let t61094 = t10254 * t17635;
    let t61098 = t13554 * t12652;
    let t61102 = 0.22222222222222222222e-2_f64 * t61065 * t61066 * t984 * t13835 - 0.37037037037037037036e-3_f64 * t47907 - 0.81481481481481481481e-2_f64 * t10263 * t5845 + 0.14814814814814814814e-2_f64 * t61074 + 0.44444444444444444444e-2_f64 * t2960 * t17844 + 0.66666666666666666665e-2_f64 * t2986 * t4518 * t61078 - 0.33333333333333333332e-2_f64 * t2986 * t2988 * t61082 - 0.17283950617283950617e-2_f64 * t2986 * t43069 * t61086 - 0.55555555555555555554e-3_f64 * t2986 * t4531 * t23547 * t1539 + 0.11111111111111111111e-2_f64 * t2986 * t2988 * t61094 - 0.88888888888888888886e-2_f64 * t2986 * t4510 * t61098;
    (t61098, t61102)
}
