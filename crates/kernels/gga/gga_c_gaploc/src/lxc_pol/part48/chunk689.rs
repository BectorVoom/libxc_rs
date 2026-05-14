//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 689/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk689<F: Float>(t10012: F, t2530: F, t2684: F, t9438: F, t40820: F, t900: F, t22624: F, t7427: F, t22634: F, t22629: F, t825: F, t9624: F, t10405: F, t2482: F, t9267: F, t3338: F, t4130: F) -> (F, F, F, F, F, F, F, F) {
    let t41316 = t2684 * t9438 * t10012 * t2530;
    let t41339 = t900 * t40820;
    let t41408 = t7427 * t9438 * t22624;
    let t41448 = t2684 * t9438 * t22634;
    let t41477 = t825 * t9438 * t22629;
    let t41511 = t900 * t9624;
    let t41588 = t9267 * t10405 * t2482;
    let t41590 = t4130 * t3338;
    (t41316, t41339, t41408, t41448, t41477, t41511, t41588, t41590)
}
