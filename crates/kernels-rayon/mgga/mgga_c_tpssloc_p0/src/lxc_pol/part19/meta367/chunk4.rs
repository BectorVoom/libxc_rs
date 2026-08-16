//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1348/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1348(t10236: f64, t9288: f64, t10186: f64, t10204: f64, t10237: f64, t10241: f64, t10245: f64, t10251: f64, t10259: f64, t13831: f64, t2960: f64, t2986: f64, t2988: f64, t2990: f64, t43038: f64, t43043: f64, t43055: f64, t43059: f64, t43061: f64, t43065: f64, t43069: f64, t43071: f64) -> f64 {
    let t43075 = t10236 * t9288;
    let t43079 = -0.29629629629629629628e-2_f64 * t2960 * t10204 + 0.37037037037037037036e-3_f64 * t43038 - 0.33333333333333333332e-2_f64 * t2986 * t10241 * t13831 + 0.66666666666666666664e-2_f64 * t2986 * t2988 * t43043 - 0.16666666666666666666e-2_f64 * t2986 * t10259 * t10245 + 0.17777777777777777777e-1_f64 * t10186 * t10251 + 0.74074074074074074072e-3_f64 * t43055 - 0.11111111111111111111e-2_f64 * t43059 - 0.11111111111111111111e-2_f64 * t2986 * t43061 * t2990 - 0.22222222222222222222e-2_f64 * t2986 * t43065 * t10237 - 0.34567901234567901234e-2_f64 * t2986 * t43069 * t43071 - 0.66666666666666666664e-2_f64 * t2986 * t2988 * t43075;
    t43079
}
