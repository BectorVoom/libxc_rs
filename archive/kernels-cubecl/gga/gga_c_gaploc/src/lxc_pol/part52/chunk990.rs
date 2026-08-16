//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 990/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk990<F: Float>(t14415: F, t2508: F, t2580: F, t45065: F, t45072: F, t45077: F, t45079: F, t45083: F, t45086: F, t45090: F, t45101: F, t45104: F, t45107: F, t45109: F, t45111: F, t45115: F, t47768: F, t47772: F, t50043: F, t50092: F, t7129: F) -> F {
    let t50465 = t45065 - t45072 - t45077 + t45079 + t45083 + t45086 + t45090 + F::cast_from(0.30762104920568897134e-1_f64) * t7129 * t14415 + F::cast_from(0.30762104920568897134e-1_f64) * t2508 * t2580 * t50043 + F::cast_from(0.30762104920568897134e-1_f64) * t2508 * t2580 * t50092 - F::cast_from(0.1281754371690370714e-2_f64) * t47768 - F::cast_from(0.1281754371690370714e-2_f64) * t47772 + t45101 + t45104 - t45107 + t45109 - t45111 + t45115;
    t50465
}
