//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1070/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1070<F: Float>(t39749: F, t446: F, t86661: F, t85483: F, t9327: F, t39725: F, t86637: F, t27: F, t40266: F, t86676: F, t89: F, t49337: F, t78362: F, t78364: F, t78366: F, t78368: F, t78396: F, t87056: F, t87060: F, t87063: F, t87067: F, t87071: F) -> (F, F, F, F, F) {
    let t87074 = t446 * t39749 * t86661;
    let t87077 = t446 * t9327 * t85483;
    let t87080 = t446 * t39725 * t86637;
    let t87084 = t89 * t27 * t40266 * t86676;
    let t87086 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t78362 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t78364 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t78366 + F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t78368 + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t87056 + F::cast_from(112.0_f64) / F::cast_from(243.0_f64) * t49337 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t87060 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t87063 + F::cast_from(40.0_f64) / F::cast_from(27.0_f64) * t87067 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t78396 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t87071 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t87074 - F::cast_from(20.0_f64) / F::cast_from(27.0_f64) * t87077 + F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t87080 + F::cast_from(8.0_f64) * t87084;
    (t87074, t87077, t87080, t87084, t87086)
}
