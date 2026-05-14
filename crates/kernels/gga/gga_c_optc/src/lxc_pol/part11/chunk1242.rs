//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1242/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1242<F: Float>(t5133: F, t2958: F, t5126: F, t26424: F, t2941: F, t26266: F, t1045: F, t58753: F, t1450: F, t52528: F, t52533: F, t14984: F, t14992: F, t43503: F, t43508: F, t44329: F, t52687: F, t52689: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t58757 = t5133 * t5133;
    let t58758 = t2958 * t58757;
    let t58760 = t5126 * t5126;
    let t58761 = t26424 * t58760;
    let t58763 = t2941 * t58757;
    let t58765 = t26266 * t58760;
    let t58770 = t1045 * t58753;
    let t58774 = t52528 * t1450;
    let t58776 = t52533 * t1450;
    let t58778 = t14984 * t5133;
    let t58780 = t14992 * t5133;
    let t58782 = 0.247573125e0 * t58758 - 0.485484375e1 * t58761 - 0.3883875e1 * t58763 + 0.6189328125e-1 * t58765 - 0.80513333333333333336e0 * t43503 + 0.16102666666666666667e1 * t43508 - 0.5519e0 * t44329 + 0.16504875e0 * t58770 + 0.22076e0 * t52687 - 0.132456e1 * t52689 - 0.51785e1 * t58774 + 0.3300975e0 * t58776 + 0.11651625e2 * t58778 - 0.247573125e0 * t58780;
    (t58758, t58761, t58763, t58765, t58770, t58774, t58776, t58778, t58780, t58782)
}
