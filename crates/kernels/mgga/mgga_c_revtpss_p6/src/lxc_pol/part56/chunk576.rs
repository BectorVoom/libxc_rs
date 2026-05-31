//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 576/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk576<F: Float>(t1149: F, t1733: F, t3384: F, t1723: F, t3390: F, t1134: F, t3358: F, t3394: F, t5044: F, t5049: F, t5054: F, t5058: F) -> (F, F, F) {
    let t5068 = t1733 * t1149;
    let t5070 = F::cast_from(2.0_f64) * t3384 * t5068;
    let t5071 = t3390 * t1723;
    let t5072 = t5071 * t1134;
    let t5079 = t3394 - t3358 / F::cast_from(9.0_f64) - t5044 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t5049 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t5054 + t5058 / F::cast_from(3.0_f64);
    (t5070, t5072, t5079)
}
