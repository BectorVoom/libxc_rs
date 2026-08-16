//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1442/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1442<F: Float>(t550: F, t816: F, t9707: F, t1379: F, t2689: F, t3952: F, t547: F, t9646: F, t2236: F, t66: F) -> (F, F, F, F) {
    let t9709 = t9707 * t550 * t816;
    let t9711 = F::cast_from(0.12846167376791569079e-2_f64) * t1379 * t9709;
    let t9712 = t2689 * t3952;
    let t9718 = t9646 * t547;
    let t9720 = F::cast_from(1.0_f64) / t66 / t2236;
    (t9711, t9712, t9718, t9720)
}
