//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 939/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk939<F: Float>(t39725: F, t446: F, t86637: F, t27: F, t40266: F, t86676: F, t89: F, t49337: F, t78362: F, t78364: F, t78366: F, t78368: F, t78396: F, t87056: F, t87060: F, t87063: F, t87067: F, t87071: F, t87074: F, t87077: F) -> (F, F, F) {
    let t87080 = t446 * t39725 * t86637;
    let t87084 = t89 * t27 * t40266 * t86676;
    let t87086 = 8.0 / 9.0 * t78362 - 4.0 / 9.0 * t78364 - 4.0 / 9.0 * t78366 + 8.0 / 27.0 * t78368 + 3.0 / 4.0 * t87056 + 112.0 / 243.0 * t49337 + 2.0 / 3.0 * t87060 + 4.0 / 9.0 * t87063 + 40.0 / 27.0 * t87067 + 8.0 / 9.0 * t78396 + 8.0 / 3.0 * t87071 + 8.0 / 3.0 * t87074 - 20.0 / 27.0 * t87077 + 40.0 / 81.0 * t87080 + 8.0 * t87084;
    (t87080, t87084, t87086)
}
