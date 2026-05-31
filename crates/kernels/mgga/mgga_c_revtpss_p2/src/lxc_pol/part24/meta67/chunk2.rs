//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 422/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk422<F: Float>(t1132: F, t1723: F, t1139: F, t1145: F, t1715: F, t141: F, t1137: F, t1144: F, t1717: F) -> (F, F, F, F, F) {
    let t1724 = t1132 * t1723;
    let t1727 = t1139 * t1723;
    let t1729 = t1145 * t1715;
    let t1730 = t141 * t1729;
    let t1732 = F::cast_from(0.1898925e1_f64) * t1724 - t1137 + F::cast_from(0.29896666666666666667e0_f64) * t1717 + F::cast_from(0.3071625e0_f64) * t1727 - t1144 + F::cast_from(0.82156666666666666667e-1_f64) * t1730;
    (t1724, t1727, t1729, t1730, t1732)
}
