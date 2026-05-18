//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1174/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1174<F: Float>(t29671: F, t8130: F, t8133: F, t1881: F, t8256: F, t637: F, t6895: F, t2233: F, t12861: F, t1607: F, t4314: F, t4455: F) -> (F, F, F, F, F, F) {
    let t29672 = t29671 / F::new(8.0);
    let t29673 = t8130 * t8133;
    let t29674 = t29673 / F::new(8.0);
    let t29675 = t1881 * t8256;
    let t29676 = t29675 / F::new(8.0);
    let t29677 = t6895 * t637;
    let t29678 = t2233 * t29677;
    let t29679 = t29678 / F::new(16.0);
    let t30409 = t1607 * t12861;
    let t30424 = t4455 * t4314;
    (t29672, t29674, t29676, t29679, t30409, t30424)
}
