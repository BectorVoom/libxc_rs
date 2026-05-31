//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2067/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2067<F: Float>(t21829: F, t665: F, t10227: F, t5895: F, t658: F, t1504: F, t2: F, t580: F, t2349: F, t5823: F, t9342: F, t100: F) -> (F, F, F, F, F, F) {
    let t21830 = t21829 * t665;
    let t21835 = t10227 * t5895;
    let t21836 = t21835 * t658;
    let t21839 = t1504 * t2;
    let t21840 = t21839 * t580;
    let t21845 = t2349 * t5823;
    let t21846 = t21845 * t658;
    let t21850 = -t580 - F::cast_from(3.0_f64) * t9342;
    let t21851 = t100 * t21850;
    (t21830, t21836, t21840, t21846, t21850, t21851)
}
