//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1113/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1113<F: Float>(t10355: F, t22688: F, t4201: F, t5825: F, t22671: F, t48: F, t477: F, t53: F, t10368: F, t4210: F, t60: F, t10379: F, t1480: F, t1483: F, t44: F, t56: F, t5843: F, t5848: F, t5851: F, t61: F, sigma2: F) -> (F, F, F, F, F, F) {
    let t22689 = t10355 * t22688;
    let t22692 = t4201 * t5825;
    let t22695 = t48 * t22671;
    let t22699 = F::new(1.0) / t53 / t477;
    let t22700 = sigma2 * t22699;
    let t22709 = t10368 * t22688;
    let t22712 = t4210 * t5825;
    let t22715 = t60 * t22671;
    let t22718 = -F::new(5.0) / F::new(108.0) * t44 * t22689 + F::new(5.0) / F::new(6.0) * t44 * t22692 + F::new(5.0) / F::new(6.0) * t44 * t22695 - F::new(1232.0) / F::new(27.0) * t22700 * t61 - F::new(220.0) / F::new(9.0) * t5843 * t1483 - F::new(20.0) / F::new(9.0) * t1480 * t5848 + F::new(20.0) / F::new(3.0) * t1480 * t5851 + F::new(5.0) / F::new(108.0) * t56 * t22709 + F::new(5.0) / F::new(6.0) * t56 * t22712 - F::new(5.0) / F::new(6.0) * t56 * t22715 + t10379;
    (t22699, t22700, t22709, t22712, t22715, t22718)
}
