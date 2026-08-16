//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 825/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk825<F: Float>(t14756: F, t27221: F, t4435: F, t7045: F, t4426: F, t7038: F, t25245: F, t4430: F, t1561: F, t25266: F, t25270: F, t4462: F) -> (F, F, F, F, F, F) {
    let t27222 = t27221 * t14756;
    let t27224 = t7045 * t4435;
    let t27226 = t7038 * t4426;
    let t27228 = t25245 * t4430;
    let t27230 = t25266 * t1561;
    let t27232 = t25270 * t4462;
    (t27222, t27224, t27226, t27228, t27230, t27232)
}
