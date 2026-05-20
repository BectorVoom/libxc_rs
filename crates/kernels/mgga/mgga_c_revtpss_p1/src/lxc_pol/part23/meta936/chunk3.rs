//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3079/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3079<F: Float>(t81379: F, t81397: F, t1132: F, t1139: F, t43771: F, t44039: F, t44040: F, t68255: F, t68257: F, t81156: F, t81158: F, t81162: F, t81167: F) -> (F, F, F) {
    let t81398 = t81379 + t81397;
    let t81399 = t1132 * t81398;
    let t81401 = t1139 * t81398;
    let t81403 = F::cast_from(0.39862222222222222223e0_f64) * t68255 - F::cast_from(0.26574814814814814815e0_f64) * t68257 - F::cast_from(0.2434271604938271605e0_f64) * t43771 + F::cast_from(0.19931111111111111111e0_f64) * t81156 - F::cast_from(0.59793333333333333333e0_f64) * t81158 + F::cast_from(0.99655555555555555554e0_f64) * t81162 + F::cast_from(0.39862222222222222223e1_f64) * t81167 + F::new(0.1898925e1) * t81399 + t44039 + t44040 + F::new(0.3071625e0) * t81401;
    (t81399, t81401, t81403)
}
