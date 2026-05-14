//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 944/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk944<F: Float>(t62846: F, t62853: F, t62856: F, t78012: F, t78015: F, t78027: F, t87024: F, t87027: F, t87030: F, t87033: F, t87037: F, t87042: F, t87045: F, t87048: F, t64516: F, t78362: F, t78364: F, t78366: F, t78368: F, t78396: F, t87060: F, t87063: F, t87067: F, t87071: F, t87074: F, t87077: F, t87080: F, t87084: F) -> (F, F) {
    let t87200 = 2.0 / 27.0 * t78012 - 8.0 / 27.0 * t78015 - 4.0 / 3.0 * t78027 - t62846 - 2.0 / 3.0 * t87024 - 2.0 / 3.0 * t87027 - 4.0 / 9.0 * t87030 - 8.0 / 9.0 * t87033 - t87037 / 6.0 - t62853 + t62856 + 4.0 / 3.0 * t87042 + 2.0 / 9.0 * t87045 + 4.0 / 3.0 * t87048;
    let t87214 = 4.0 / 9.0 * t78362 - 2.0 / 9.0 * t78364 - 2.0 / 9.0 * t78366 + 4.0 / 27.0 * t78368 + t64516 + t87060 / 3.0 + 2.0 / 9.0 * t87063 + 20.0 / 27.0 * t87067 + 4.0 / 9.0 * t78396 + 4.0 / 3.0 * t87071 + 4.0 / 3.0 * t87074 - 10.0 / 27.0 * t87077 + 20.0 / 81.0 * t87080 + 4.0 * t87084;
    (t87200, t87214)
}
