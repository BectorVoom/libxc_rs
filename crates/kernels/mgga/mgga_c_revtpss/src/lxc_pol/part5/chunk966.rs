//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 966/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk966<F: Float>(t12295: F, t12351: F, t1159: F, t3475: F, t426: F, t3478: F, t434: F, t1175: F, t3520: F, t3519: F, t444: F, t439: F, t3495: F, t1156: F, t3451: F, t1178: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12459 = 0.16068111111111111111e1 * t12295;
    let t12460 = 0.46308888888888888888e0 * t12351;
    let t12469 = 1.0 / t3475 / t1159;
    let t12470 = t426 * t12469;
    let t12472 = 1.0 / t3478 / t434;
    let t12481 = t1175 * t3520;
    let t12485 = 1.0 / t3519 / t444;
    let t12486 = t439 * t12485;
    let t12491 = t1175 * t3495;
    let t12511 = t1156 * t3451;
    let t12542 = 0.93932222222222222223e0 * t12295;
    let t12543 = 0.36793333333333333333e0 * t12351;
    let t12552 = 1.0 / t3519 / t1178;
    (t12459, t12460, t12470, t12472, t12481, t12485, t12486, t12491, t12511, t12542, t12543, t12552)
}
