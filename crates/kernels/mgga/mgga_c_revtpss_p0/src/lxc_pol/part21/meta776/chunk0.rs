//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2766/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2766<F: Float>(t4302: F, t9586: F, t13312: F, t189: F, t4401: F, t606: F, t14389: F, t2258: F, t10612: F, t4311: F, t14330: F, t14369: F, t2251: F) -> (F, F, F, F, F) {
    let t50856 = t4302 * t9586;
    let t50857 = F::cast_from(0.56968947174242584612e-3_f64) * t50856;
    let t50861 = F::new(36.0) * t4401 * t189 * t13312 * t606;
    let t50864 = F::new(36.0) * t4401 * t14389 * t2258;
    let t50865 = t4311 * t10612;
    let t50866 = F::new(12.0) * t50865;
    let t50868 = t14330 * t14369 * t2251;
    (t50857, t50861, t50864, t50866, t50868)
}
