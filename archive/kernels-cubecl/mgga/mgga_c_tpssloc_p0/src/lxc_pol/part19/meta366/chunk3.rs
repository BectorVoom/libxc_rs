//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1335/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1335<F: Float>(t10186: F, t10241: F, t10245: F, t10256: F, t10328: F, t2960: F, t2986: F, t2988: F, t41644: F, t41649: F, t41705: F, t41715: F, t42794: F, t42799: F, t42811: F, t42817: F, t4510: F, t4518: F) -> F {
    let t42824 = -F::cast_from(0.17777777777777777777e-1_f64) * t10186 * t10256 + F::cast_from(0.22222222222222222222e-2_f64) * t42794 - F::cast_from(0.16666666666666666666e-2_f64) * t2986 * t10241 * t10245 - F::cast_from(0.11111111111111111111e-2_f64) * t2986 * t2988 * t42799 + F::cast_from(0.99999999999999999996e-2_f64) * t2986 * t4518 * t41715 + F::cast_from(0.14814814814814814815e-2_f64) * t2986 * t4510 * t41705 + F::cast_from(0.88888888888888888888e-2_f64) * t2960 * t10328 - F::cast_from(0.32921810699588477364e-2_f64) * t42811 - t42817 - F::cast_from(0.22222222222222222221e-2_f64) * t2986 * t4518 * t41644 - F::cast_from(0.13333333333333333333e-1_f64) * t2986 * t4510 * t41649;
    t42824
}
