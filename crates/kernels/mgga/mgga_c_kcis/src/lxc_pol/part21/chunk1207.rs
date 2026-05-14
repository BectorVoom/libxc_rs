//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1207/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1207<F: Float>(t96261: F, t96270: F, t1262: F, t26996: F, t5329: F, t5341: F, t13173: F, t15534: F, t26955: F, t26960: F, t27010: F, t28116: F, t28137: F, t28190: F, t7772: F, t7788: F, t92761: F, t93143: F, t93145: F, t96256: F, t96259: F, t96720: F, t97141: F) -> (F, F) {
    let t97352 = 0.61905925925925925925e-2 * t96261;
    let t97360 = 0.61905925925925925925e-2 * t96270;
    let t97366 = t5329 * t26996 * t5341 * t1262;
    let t97371 = 0.77382407407407407407e-3 * t96256 - 0.185671721767578125e-4 * t92761 * t28137 - 0.34822083333333333332e-2 * t96259 + t97352 + 0.92673611111111111112e-3 * t26960 * t15534 * t28116 * t13173 + 0.61836467013888888889e-4 * t26955 * t97141 + 0.11349419753086419753e-1 * t93143 - t97360 - 0.77382407407407407406e-3 * t93145 + 0.208515625e-2 * t7788 * t96720 - 0.92754700520833333334e-4 * t7772 * t97366 - 0.11584201388888888889e-3 * t28190 * t27010;
    (t97366, t97371)
}
