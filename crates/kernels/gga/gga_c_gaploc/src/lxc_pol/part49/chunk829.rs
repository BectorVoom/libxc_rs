//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 829/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk829<F: Float>(t11392: F, t3159: F, t10348: F, t10485: F, t2386: F, t3338: F, t544: F, t6514: F, t40549: F, t40555: F, t40558: F, t40561: F, t40567: F, t40570: F, t2902: F, t3145: F, t4349: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t42444 = 0.25025342966295298669e1 * t3159 * t11392;
    let t42448 = t10485 * t10348;
    let t42452 = t544 * t6514 * t3338 * t2386;
    let t42455 = 0.11916829983950142223e0 * t40549;
    let t42456 = 0.89376224879626066674e-1 * t40555;
    let t42457 = 0.59584149919750711116e-1 * t40558;
    let t42458 = 0.59584149919750711116e-1 * t40561;
    let t42460 = 0.25561950635947166451e1 * t40567;
    let t42461 = 0.29792074959875355558e-1 * t40570;
    let t42470 = 6.0 * t4349 * t2902 * t3145;
    (t42444, t42448, t42452, t42455, t42456, t42457, t42458, t42460, t42461, t42470)
}
