//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1348/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1348<F: Float>(t10236: F, t9288: F, t10186: F, t10204: F, t10237: F, t10241: F, t10245: F, t10251: F, t10259: F, t13831: F, t2960: F, t2986: F, t2988: F, t2990: F, t43038: F, t43043: F, t43055: F, t43059: F, t43061: F, t43065: F, t43069: F, t43071: F) -> F {
    let t43075 = t10236 * t9288;
    let t43079 = -F::cast_from(0.29629629629629629628e-2_f64) * t2960 * t10204 + F::cast_from(0.37037037037037037036e-3_f64) * t43038 - F::cast_from(0.33333333333333333332e-2_f64) * t2986 * t10241 * t13831 + F::cast_from(0.66666666666666666664e-2_f64) * t2986 * t2988 * t43043 - F::cast_from(0.16666666666666666666e-2_f64) * t2986 * t10259 * t10245 + F::cast_from(0.17777777777777777777e-1_f64) * t10186 * t10251 + F::cast_from(0.74074074074074074072e-3_f64) * t43055 - F::cast_from(0.11111111111111111111e-2_f64) * t43059 - F::cast_from(0.11111111111111111111e-2_f64) * t2986 * t43061 * t2990 - F::cast_from(0.22222222222222222222e-2_f64) * t2986 * t43065 * t10237 - F::cast_from(0.34567901234567901234e-2_f64) * t2986 * t43069 * t43071 - F::cast_from(0.66666666666666666664e-2_f64) * t2986 * t2988 * t43075;
    t43079
}
