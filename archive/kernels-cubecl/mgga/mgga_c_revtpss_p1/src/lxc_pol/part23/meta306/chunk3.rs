//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1573/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1573<F: Float>(t3800: F, t498: F, t12295: F, t1207: F, t456: F) -> (F, F, F, F, F) {
    let t12587 = F::cast_from(1.0_f64) / t3800 / t498;
    let t12610 = F::cast_from(0.46096296296296296297e-1_f64) * t12295;
    let t12625 = t1207 * t1207;
    let t12626 = F::cast_from(1.0_f64) / t12625;
    let t12627 = t456 * t12626;
    (t12587, t12610, t12625, t12626, t12627)
}
