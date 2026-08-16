//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1664/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1664<F: Float>(t19731: F, t550: F, t1380: F, t3792: F, t5286: F, t5335: F, t1824: F, t1834: F, t5250: F, t562: F, t6387: F) -> (F, F, F, F, F, F, F) {
    let t19732 = t19731 * t550;
    let t19733 = t1380 * t19732;
    let t19735 = t3792 * t5286;
    let t19736 = t5335 * t19735;
    let t19739 = t1834 * t1824;
    let t19740 = t19739 * t5250;
    let t19743 = t562 * t6387;
    (t19732, t19733, t19735, t19736, t19739, t19740, t19743)
}
