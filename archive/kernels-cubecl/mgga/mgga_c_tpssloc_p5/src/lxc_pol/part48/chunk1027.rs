//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 1027/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk1027<F: Float>(t112620: F, t112621: F, t112622: F, t115681: F, t115684: F, t115690: F, t115695: F, t115698: F, t115700: F, t115702: F, t2040: F, t23951: F, t24028: F, t24169: F, t24545: F, t24932: F, t27888: F, t7042: F, t7050: F, t7057: F, t8690: F, t94248: F, t96222: F) -> F {
    let t117622 = -F::cast_from(2.0_f64) * t2040 * t94248 - F::cast_from(4.0_f64) * t2040 * t96222 - t23951 * t8690 - F::cast_from(2.0_f64) * t24028 * t8690 + F::cast_from(2.0_f64) * t24169 * t8690 - F::cast_from(4.0_f64) * t24545 * t7042 - F::cast_from(4.0_f64) * t24932 * t7057 - F::cast_from(4.0_f64) * t27888 * t7050 - t112620 - t112621 - t112622 + t115681 + t115684 + t115690 + t115695 - t115698 + t115700 - t115702;
    t117622
}
