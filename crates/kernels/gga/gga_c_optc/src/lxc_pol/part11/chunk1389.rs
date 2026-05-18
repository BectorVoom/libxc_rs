//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1389/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1389<F: Float>(t14992: F, t5133: F, t43503: F, t43508: F, t44329: F, t52687: F, t52689: F, t58758: F, t58761: F, t58763: F, t58765: F, t58770: F, t58774: F, t58776: F, t58778: F) -> (F, F) {
    let t58780 = t14992 * t5133;
    let t58782 = F::new(0.247573125e0) * t58758 - F::new(0.485484375e1) * t58761 - F::new(0.3883875e1) * t58763 + F::new(0.6189328125e-1) * t58765 - F::new(0.80513333333333333336e0) * t43503 + F::new(0.16102666666666666667e1) * t43508 - F::new(0.5519e0) * t44329 + F::new(0.16504875e0) * t58770 + F::new(0.22076e0) * t52687 - F::new(0.132456e1) * t52689 - F::new(0.51785e1) * t58774 + F::new(0.3300975e0) * t58776 + F::new(0.11651625e2) * t58778 - F::new(0.247573125e0) * t58780;
    (t58780, t58782)
}
