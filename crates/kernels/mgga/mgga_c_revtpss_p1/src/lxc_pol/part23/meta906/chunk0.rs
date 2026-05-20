//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2912/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2912<F: Float>(t324: F, t77549: F, t77596: F, t300: F, t1633: F, t52894: F, t64043: F, t972: F, t19331: F, t52514: F, t1610: F, t63610: F) -> (F, F, F, F, F) {
    let t77598 = (t77549 + t77596) * t324;
    let t77600 = F::cast_from(0.19751673498613801407e-1_f64) * t300 * t77598;
    let t77604 = F::cast_from(0.30762056574649219973e4_f64) * t52894 * t64043 * t1633 * t972;
    let t77612 = F::cast_from(0.2894756309764656312e3_f64) * t52514 * t19331;
    let t77622 = F::new(3.0) * t63610 * t1610;
    (t77598, t77600, t77604, t77612, t77622)
}
