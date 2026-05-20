//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1822/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1822<F: Float>(t48324: F, t187: F, t92011: F, t48331: F, t48333: F, t48335: F, t40076: F, t40079: F, t47124: F, t47131: F, t47138: F, t47140: F, t47142: F, t47152: F) -> (F, F, F, F, F, F) {
    let t92024 = F::cast_from(0.1301229756036208781e0_f64) * t48324;
    let t92026 = F::cast_from(0.19751673498613801407e-1_f64) * t92011 * t187;
    let t92027 = F::new(384.0) * t48331;
    let t92028 = F::new(144.0) * t48333;
    let t92029 = F::cast_from(0.4155806185363551302e3_f64) * t48335;
    let t92030 = t47124 + t47131 - t47138 - t47140 + t47142 - t92024 + t40076 - t40079 + t92026 + t92027 + t47152 + t92028 + t92029;
    (t92024, t92026, t92027, t92028, t92029, t92030)
}
