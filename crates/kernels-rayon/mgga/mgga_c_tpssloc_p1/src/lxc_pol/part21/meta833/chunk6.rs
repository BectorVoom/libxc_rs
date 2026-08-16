//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2947/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2947(t4509: f64, t5842: f64, t17686: f64, t42841: f64, t17783: f64, t2960: f64, t13779: f64, t17167: f64, t2986: f64, t10235: f64, t10237: f64, t10241: f64, t10245: f64, t10263: f64, t17804: f64, t17817: f64, t17863: f64, t42846: f64, t4518: f64, t48281: f64, t5818: f64, t5825: f64, t59659: f64) -> f64 {
    let t61365 = t4509 * t5842;
    let t61375 = t42841 * t17686;
    let t61383 = t2960 * t17783;
    let t61387 = t2986 * t13779 * t17167;
    let t61389 = 0.55555555555555555554e-3_f64 * t2986 * t10241 * t17817 - 0.27777777777777777777e-3_f64 * t2986 * t17804 * t10245 - 0.37037037037037037036e-3_f64 * t2986 * t61365 * t10237 - 0.66666666666666666664e-2_f64 * t2986 * t4518 * t59659 - 0.37037037037037037036e-3_f64 * t2986 * t42846 * t17863 + 0.44444444444444444442e-2_f64 * t2986 * t10235 * t61375 - 0.54320987654320987651e-2_f64 * t10263 * t5825 + 0.36213991769547325102e-2_f64 * t10263 * t5818 - 0.6584362139917695473e-3_f64 * t61383 - 0.24691358024691358024e-3_f64 * t48281 + 0.11111111111111111111e-2_f64 * t61387;
    t61389
}
