//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1121/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1121<F: Float>(t4135: F, t4147: F, t2034: F, t2014: F, t10416: F, t1936: F, t13435: F, t2322: F, t7002: F, t13440: F, t5523: F, t112: F, t239: F) -> (F, F, F, F, F, F, F, F, F) {
    let t25802 = t4147 * t4135;
    let t25803 = t2034 * t25802;
    let t25804 = t2014 * t25803;
    let t25812 = F::new(2.0) * t10416 * t1936;
    let t25814 = F::new(4.0) * t13435 * t1936;
    let t25816 = F::new(4.0) * t2322 * t7002;
    let t25818 = F::new(2.0) * t13440 * t1936;
    let t25820 = F::new(4.0) * t5523 * t7002;
    let t25821 = t239 * t112;
    (t25802, t25803, t25804, t25812, t25814, t25816, t25818, t25820, t25821)
}
