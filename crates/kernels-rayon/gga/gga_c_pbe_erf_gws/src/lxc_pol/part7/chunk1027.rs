//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1027/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1027(t1447: f64, t156: f64, t4835: f64, t4841: f64, t4850: f64, t4844: f64, t4782: f64, t4847: f64, t4788: f64, t4838: f64, t1396: f64, t542: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t18587 = 0.13012297059337829057e0_f64 * t1447 * t156 * t4835;
    let t18588 = t4850 * t4841;
    let t18589 = 0.13012297059337829057e0_f64 * t18588;
    let t18590 = t4850 * t4844;
    let t18591 = 0.1926377843805564792e1_f64 * t18590;
    let t18594 = 0.38024868119570572865e2_f64 * t1447 * t156 * t4782;
    let t18595 = t4850 * t4847;
    let t18596 = 0.65061485296689145286e-1_f64 * t18595;
    let t18599 = 0.21687161765563048428e-1_f64 * t1447 * t156 * t4788;
    let t18600 = t4850 * t4838;
    let t18601 = 0.86748647062252193714e-1_f64 * t18600;
    let t18604 = 0.43374323531126096856e-1_f64 * t1447 * t542 * t1396;
    (t18587, t18589, t18591, t18594, t18596, t18599, t18601, t18604)
}
