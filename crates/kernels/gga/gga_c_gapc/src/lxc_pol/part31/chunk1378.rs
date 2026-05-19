//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1378/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1378<F: Float>(t33717: F, t33726: F, t33728: F, t33731: F, t33734: F, t33741: F, t33743: F, t33746: F, t33750: F, t33753: F, t33755: F, t33758: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t36676 = F::cast_from(0.73744819641113281254e-8_f64) * t33717;
    let t36678 = F::cast_from(0.40481770833333333336e-4_f64) * t33726;
    let t36679 = F::cast_from(0.11372686522837130914e-5_f64) * t33728;
    let t36680 = F::cast_from(0.11372686522837130914e-5_f64) * t33731;
    let t36681 = F::cast_from(0.4637672555408563478e-4_f64) * t33734;
    let t36687 = F::cast_from(0.43284943850479925795e-3_f64) * t33741;
    let t36688 = F::cast_from(0.1351988360087076823e-6_f64) * t33743;
    let t36689 = F::cast_from(0.21102562238076876322e-7_f64) * t33746;
    let t36690 = F::cast_from(0.40021712703254065176e-7_f64) * t33750;
    let t36691 = F::cast_from(0.80043425406508130352e-7_f64) * t33753;
    let t36692 = F::cast_from(0.32826207925897363168e-8_f64) * t33755;
    let t36693 = F::cast_from(0.49520679385353736436e-5_f64) * t33758;
    (t36676, t36678, t36679, t36680, t36681, t36687, t36688, t36689, t36690, t36691, t36692, t36693)
}
