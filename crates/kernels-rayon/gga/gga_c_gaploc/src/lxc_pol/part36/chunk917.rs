//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 917/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk917(t2268: f64, t2343: f64, t41869: f64, t12767: f64, t6305: f64, t1063: f64, t3158: f64, t8207: f64, t2304: f64, t34273: f64, t39849: f64, t12803: f64, t29874: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42835 = t2268 * t2343 * t41869;
    let t42838 = 0.56910013271352299198e-1_f64 * t6305 * t12767;
    let t42841 = 0.19918504644973304719e0_f64 * t1063 * t3158 * t8207;
    let t42844 = 0.39837009289946609438e0_f64 * t2268 * t2304 * t34273;
    let t42845 = 0.142275033178380748e-1_f64 * t39849;
    let t42846 = t29874 * t12803;
    (t42835, t42838, t42841, t42844, t42845, t42846)
}
