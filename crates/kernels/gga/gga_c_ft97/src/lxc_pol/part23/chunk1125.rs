//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1125/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1125<F: Float>(t108685: F, t6043: F, t6046: F, t24378: F, t27665: F, t6034: F, t14842: F, t173: F, t27670: F, t27671: F, t17817: F, t96694: F, t3817: F, t703: F, t24330: F, t27588: F) -> (F, F, F, F, F, F) {
    let t108688 = 0.6809984893827160494e-1 * t6043 * t108685 * t6046;
    let t108697 = 0.14846767889314528222e-3 * t6034 * t24378 * t27665;
    let t108733 = t27670 * t27671 * t173 * t14842;
    let t108738 = t17817 * t96694;
    let t108761 = t703 * t3817;
    let t108781 = 0.25537443351851851852e-1 * t6043 * t24330 * t27588;
    (t108688, t108697, t108733, t108738, t108761, t108781)
}
