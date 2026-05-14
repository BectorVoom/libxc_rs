//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1055/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1055<F: Float>(t1631: F, t929: F, t22642: F, t22819: F, t25793: F, t25802: F, t415: F, t5569: F, t22613: F, t25649: F, t1293: F, t1594: F, t25754: F, t420: F, t49004: F, t5598: F, t6445: F, t92433: F) -> (F, F, F, F, F, F, F, F) {
    let t100850 = t1631 * t929;
    let t100880 = 0.60548059007656442388e-3 * t22819 * t22642 * t25793;
    let t100910 = 0.29693535778629056444e-4 * t5569 * t415 * t25802;
    let t100980 = 0.29693535778629056444e-3 * t22613 * t415 * t25649;
    let t100999 = t1293 * t929;
    let t101026 = t1594 * t929;
    let t101098 = t25754 * t420 * t49004;
    let t101139 = 0.68099848938271604939e-1 * t5598 * t92433 * t6445;
    (t100850, t100880, t100910, t100980, t100999, t101026, t101098, t101139)
}
