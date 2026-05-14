//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1123/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1123<F: Float>(t384: F, t58286: F, t58513: F, t29468: F, t7878: F, t101466: F, t115312: F, t115316: F, t115333: F, t115349: F, t115353: F, t115389: F, t115410: F, t22552: F, t22583: F, t22597: F, t22603: F, t22743: F, t22761: F, t22819: F, t29483: F, t2984: F, t37551: F, t4491: F, t53: F, t5538: F, t5540: F, t5579: F, t5591: F, t72: F, t73881: F, t74254: F, t92367: F, t92897: F, t92899: F, t930: F) -> (F, F, F, F) {
    let t115432 = t58286 * t384;
    let t115436 = t58513 * t384;
    let t115440 = t29468 * t7878;
    let t115477 = 0.19795690519086037629e-3 * t22583 * t101466 * t930 * t2984 - 0.98910212891072794758e-5 * t92897 * t92899 * t115410 - 0.51690243689028715488e-5 * t5538 * t5540 * t115432 - 0.1721820212247325051e-5 * t5538 * t22743 * t115436 - 0.3443640424494650102e-5 * t22597 * t22743 * t115440 + 0.1721820212247325051e-5 * t22603 * t22743 * t115353 + 0.10330921273483950306e-5 * t5538 * t22743 * t115349 - 0.3443640424494650102e-5 * t5538 * t22743 * t115389 + 0.90822088511484663582e-3 * t22819 * t5591 * t72 * t4491 * t53 + 0.51690243689028715488e-4 * t22597 * t5540 * t115312 - 0.25845121844514357744e-4 * t22603 * t5540 * t115316 + 0.51690243689028715487e-4 * t37551 * t5540 * t115333 + 0.76612330055555555556e-1 * t22552 * t5579 * t72 * t73881 - 0.11491849508333333333e0 * t22761 * t5579 * t72 * t74254 + 0.13810404665630505674e-4 * t29483 * t92367;
    (t115432, t115436, t115440, t115477)
}
