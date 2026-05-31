//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2160/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2160<F: Float>(t92963: F, t92966: F, t92969: F, t27253: F, t9775: F, t14833: F, t240: F, t2661: F, t7043: F, t14853: F, t7045: F, t14857: F, t25234: F) -> (F, F, F, F, F, F, F) {
    let t98960 = F::cast_from(0.10164000561857065645e-4_f64) * t92963;
    let t98961 = F::cast_from(0.72286371995927450868e-4_f64) * t92966;
    let t98962 = F::cast_from(35.0_f64) / F::cast_from(108.0_f64) * t92969;
    let t98964 = t9775 * t27253;
    let t98968 = t2661 * t7043 * t240 * t14833;
    let t98970 = t7045 * t14853;
    let t98972 = t25234 * t14857;
    (t98960, t98961, t98962, t98964, t98968, t98970, t98972)
}
