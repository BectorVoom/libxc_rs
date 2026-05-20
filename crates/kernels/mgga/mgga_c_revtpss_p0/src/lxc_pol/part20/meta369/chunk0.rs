//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1344/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1344<F: Float>(t10428: F, t2414: F, t10587: F, t2496: F, t10467: F, t705: F, t707: F, t190: F, t39457: F, t706: F, t39875: F, t39894: F, t9371: F) -> (F, F, F, F, F) {
    let t40155 = F::new(24.0) * t10428 * t2414;
    let t40156 = t10587 * t2496;
    let t40157 = F::cast_from(0.10389515463408878255e3_f64) * t40156;
    let t40158 = t705 * t10467;
    let t40160 = F::new(16.0) * t40158 * t707;
    let t40163 = F::new(4.0) * t706 * t190 * t39457;
    let t40165 = t39894 * t39875 * t9371;
    (t40155, t40157, t40160, t40163, t40165)
}
