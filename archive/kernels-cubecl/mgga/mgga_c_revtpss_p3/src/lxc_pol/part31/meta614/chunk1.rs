//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2058/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2058<F: Float>(t25260: F, t4368: F, t820: F, t844: F, t4462: F, t92951: F, t27253: F, t9775: F, t14833: F, t240: F, t2661: F, t7043: F) -> (F, F, F, F) {
    let t98937 = t820 * t25260 * t844 * t4368;
    let t98949 = t92951 * t4462;
    let t98950 = F::cast_from(0.16006300097412701803e-1_f64) * t98949;
    let t98964 = t9775 * t27253;
    let t98968 = t2661 * t7043 * t240 * t14833;
    (t98937, t98950, t98964, t98968)
}
