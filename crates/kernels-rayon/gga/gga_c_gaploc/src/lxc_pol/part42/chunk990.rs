//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 990/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk990(t14415: f64, t2508: f64, t2580: f64, t45065: f64, t45072: f64, t45077: f64, t45079: f64, t45083: f64, t45086: f64, t45090: f64, t45101: f64, t45104: f64, t45107: f64, t45109: f64, t45111: f64, t45115: f64, t47768: f64, t47772: f64, t50043: f64, t50092: f64, t7129: f64) -> f64 {
    let t50465 = t45065 - t45072 - t45077 + t45079 + t45083 + t45086 + t45090 + 0.30762104920568897134e-1_f64 * t7129 * t14415 + 0.30762104920568897134e-1_f64 * t2508 * t2580 * t50043 + 0.30762104920568897134e-1_f64 * t2508 * t2580 * t50092 - 0.1281754371690370714e-2_f64 * t47768 - 0.1281754371690370714e-2_f64 * t47772 + t45101 + t45104 - t45107 + t45109 - t45111 + t45115;
    t50465
}
