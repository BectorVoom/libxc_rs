//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 978/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk978<F: Float>(t12485: F, t439: F, t12295: F, t12351: F, t1178: F, t3519: F) -> (F, F, F, F) {
    let t12486 = t439 * t12485;
    let t12542 = F::cast_from(0.93932222222222222223e0_f64) * t12295;
    let t12543 = F::cast_from(0.36793333333333333333e0_f64) * t12351;
    let t12552 = F::cast_from(1.0_f64) / t3519 / t1178;
    (t12486, t12542, t12543, t12552)
}
