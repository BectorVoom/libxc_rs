//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1030/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1030<F: Float>(t3475: F, t431: F, t426: F, t12295: F, t12351: F, t1159: F, t3478: F, t434: F, t1175: F, t3520: F, t3519: F, t444: F) -> (F, F, F, F, F, F, F) {
    let t12428 = F::new(1.0) / t3475 / t431;
    let t12429 = t426 * t12428;
    let t12459 = F::new(0.16068111111111111111e1) * t12295;
    let t12460 = F::new(0.46308888888888888888e0) * t12351;
    let t12469 = F::new(1.0) / t3475 / t1159;
    let t12470 = t426 * t12469;
    let t12472 = F::new(1.0) / t3478 / t434;
    let t12481 = t1175 * t3520;
    let t12485 = F::new(1.0) / t3519 / t444;
    (t12429, t12459, t12460, t12470, t12472, t12481, t12485)
}
