//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2630/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2630<F: Float>(t23160: F, t836: F, t10529: F, t2782: F, t14520: F, t14606: F, t6016: F, t860: F, t231: F, t2783: F, t18657: F, t686: F, t72: F, t874: F) -> (F, F, F, F, F) {
    let t62604 = t23160 * t836;
    let t62606 = t2782 * t10529 * t62604;
    let t62609 = t14606 * t14520;
    let t62612 = t860 * t6016;
    let t62615 = t2782 * t2783 * t62612 * t231;
    let t62619 = t874 * t18657 * t72 * t686;
    (t62606, t62609, t62612, t62615, t62619)
}
