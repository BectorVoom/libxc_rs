//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1819/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1819<F: Float>(t33: F, t21918: F, t22783: F, t3841: F, t47040: F, t516: F, t5557: F, t6416: F, t89780: F, t91811: F, t91816: F, t162: F, t91997: F, zeta_threshold: F) -> F {
    let t34 = t33 <= zeta_threshold;
    let t92009 = piecewise3::<F>(t34, F::cast_from(0.0_f64), F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t47040 * t91811 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t21918 * t6416 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t3841 * t91816 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t5557 * t22783 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t516 * t89780);
    let t92011 = (t91997 + t92009) * t162;
    t92011
}
