//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1031/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1031<F: Float>(t12485: F, t439: F, t1175: F, t3495: F, t1156: F, t3451: F, t12295: F, t12351: F, t1178: F, t3519: F, t3522: F, t447: F) -> (F, F, F, F, F, F, F, F) {
    let t12486 = t439 * t12485;
    let t12491 = t1175 * t3495;
    let t12511 = t1156 * t3451;
    let t12542 = F::new(0.93932222222222222223e0) * t12295;
    let t12543 = F::new(0.36793333333333333333e0) * t12351;
    let t12552 = F::new(1.0) / t3519 / t1178;
    let t12553 = t439 * t12552;
    let t12555 = F::new(1.0) / t3522 / t447;
    (t12486, t12491, t12511, t12542, t12543, t12552, t12553, t12555)
}
