//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1027/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1027<F: Float>(t1447: F, t156: F, t4835: F, t4841: F, t4850: F, t4844: F, t4782: F, t4847: F, t4788: F, t4838: F, t1396: F, t542: F) -> (F, F, F, F, F, F, F, F) {
    let t18587 = F::cast_from(0.13012297059337829057e0_f64) * t1447 * t156 * t4835;
    let t18588 = t4850 * t4841;
    let t18589 = F::cast_from(0.13012297059337829057e0_f64) * t18588;
    let t18590 = t4850 * t4844;
    let t18591 = F::cast_from(0.1926377843805564792e1_f64) * t18590;
    let t18594 = F::cast_from(0.38024868119570572865e2_f64) * t1447 * t156 * t4782;
    let t18595 = t4850 * t4847;
    let t18596 = F::cast_from(0.65061485296689145286e-1_f64) * t18595;
    let t18599 = F::cast_from(0.21687161765563048428e-1_f64) * t1447 * t156 * t4788;
    let t18600 = t4850 * t4838;
    let t18601 = F::cast_from(0.86748647062252193714e-1_f64) * t18600;
    let t18604 = F::cast_from(0.43374323531126096856e-1_f64) * t1447 * t542 * t1396;
    (t18587, t18589, t18591, t18594, t18596, t18599, t18601, t18604)
}
