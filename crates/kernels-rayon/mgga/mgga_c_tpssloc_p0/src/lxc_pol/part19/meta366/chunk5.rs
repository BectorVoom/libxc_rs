//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1337/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1337(t42841: f64, t9288: f64, t3014: f64, t4509: f64, t10273: f64, t2960: f64, t10231: f64, t10279: f64, t973: f64, t10186: f64, t10235: f64, t10237: f64, t10238: f64, t10242: f64, t13798: f64, t2986: f64, t2991: f64, t41693: f64, t42827: f64, t42830: f64, t42833: f64, t42839: f64) -> f64 {
    let t42842 = t42841 * t9288;
    let t42846 = t4509 * t3014;
    let t42855 = t2960 * t10273;
    let t42858 = t973 * t10231 * t10279;
    let t42860 = 0.14814814814814814815e-2_f64 * t42827 - 0.32592592592592592592e-1_f64 * t42830 * t2991 + 0.59259259259259259256e-2_f64 * t42833 + 0.11851851851851851852e-1_f64 * t10186 * t10238 - 0.14814814814814814814e-2_f64 * t42839 + 0.88888888888888888886e-2_f64 * t2986 * t10235 * t42842 - 0.22222222222222222222e-2_f64 * t2986 * t42846 * t10237 + 0.88888888888888888887e-2_f64 * t10186 * t10242 + 0.51851851851851851851e-2_f64 * t2986 * t13798 * t41693 + 0.59259259259259259256e-2_f64 * t42855 - 0.29629629629629629628e-2_f64 * t42858;
    t42860
}
