//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 736/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk736<F: Float>(t13516: F, t64: F, t40612: F, t40614: F, t40620: F, t40630: F, t40632: F, t40634: F, t43027: F, t13624: F, t1841: F, t2536: F, t734: F, t43093: F, t43100: F, t1897: F, t35583: F, t954: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t44857 = 4.0 / 3.0 * t13516 * t64;
    let t44860 = 7.0 / 256.0 * t40612;
    let t44861 = 63.0 / 8192.0 * t40614;
    let t44862 = 63.0 / 524288.0 * t40620;
    let t44863 = 21.0 / 524288.0 * t40630;
    let t44864 = 21.0 / 8192.0 * t40632;
    let t44865 = 7.0 / 768.0 * t40634;
    let t44883 = 0.1281754371690370714e-2 * t43027;
    let t44887 = 0.85450291446024714263e-3 * t1841 * t2536 * t13624 * t734;
    let t44895 = 0.1281754371690370714e-2 * t43093;
    let t44898 = 0.1281754371690370714e-2 * t43100;
    let t44901 = 0.76905262301422242837e-2 * t1897 * t954 * t35583;
    (t44857, t44860, t44861, t44862, t44863, t44864, t44865, t44883, t44887, t44895, t44898, t44901)
}
