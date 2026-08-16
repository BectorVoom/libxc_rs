//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 881/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk881(t12830: f64, t29874: f64, t39805: f64, t39808: f64, t39811: f64, t12803: f64, t1358: f64, t12797: f64, t12767: f64, t6305: f64, t1063: f64, t3158: f64, t8207: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t42820 = t29874 * t12830;
    let t42821 = 0.71137516589190373998e-2_f64 * t42820;
    let t42822 = 0.16598753870811087267e-1_f64 * t39805;
    let t42823 = 0.23712505529730124666e-2_f64 * t39808;
    let t42824 = 0.23712505529730124666e-2_f64 * t39811;
    let t42825 = t1358 * t12803;
    let t42826 = 0.63233348079280332443e-2_f64 * t42825;
    let t42827 = t29874 * t12797;
    let t42828 = 0.23712505529730124666e-2_f64 * t42827;
    let t42838 = 0.56910013271352299198e-1_f64 * t6305 * t12767;
    let t42841 = 0.19918504644973304719e0_f64 * t1063 * t3158 * t8207;
    (t42821, t42822, t42823, t42824, t42826, t42828, t42838, t42841)
}
