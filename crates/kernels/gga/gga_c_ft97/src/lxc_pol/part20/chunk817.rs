//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 817/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk817<F: Float>(t231: F, t2726: F, t24330: F, t6249: F, t6250: F, t1471: F, t2688: F, t2719: F, t6045: F, t2735: F, t1472: F, t24287: F, t1408: F, t1420: F, t14760: F, t24295: F, t24299: F, t25092: F, t25106: F, t25112: F, t2689: F, t4094: F, t6242: F, t6256: F) -> (F, F, F, F, F, F, F) {
    let t25113 = t231 * t2726;
    let t25118 = t6249 * t24330 * t6250;
    let t25120 = t2688 * t1471;
    let t25123 = t231 * t2719;
    let t25124 = t6045 * t25123;
    let t25127 = t231 * t2735;
    let t25132 = 0.11113000182098765433e-1 * t1472 * t24287;
    let t25133 = -0.45306850413028723348e0 * t2689 * t1408 + 0.45306850413028723348e0 * t14760 * t1408 + 0.45306850413028723348e0 * t4094 * t25092 - 0.22226000364197530865e-1 * t6256 * t24299 - 0.11113000182098765433e-1 * t25106 - 0.16669500273148148149e-1 * t6256 * t24295 - 0.30005100491666666667e0 * t25112 * t6045 * t25113 + 0.66678001092592592595e-1 * t25118 - 0.10001700163888888889e0 * t25120 * t1420 - 0.10001700163888888889e0 * t6242 * t25124 + 0.10001700163888888889e0 * t6249 * t6045 * t25127 + t25132;
    (t25113, t25118, t25120, t25123, t25127, t25132, t25133)
}
