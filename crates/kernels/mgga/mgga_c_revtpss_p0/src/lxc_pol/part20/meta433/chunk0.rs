//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1632/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1632<F: Float>(t1261: F, t12944: F, t3172: F, t12932: F, t3711: F, t221: F, t461: F, t462: F, t624: F, t1250: F, t606: F, t1248: F, t2258: F) -> (F, F, F, F) {
    let t44789 = t1261 * t3172 * t12944;
    let t44792 = t3711 * t3172 * t12932;
    let t44797 = F::cast_from(5.0_f64) / F::cast_from(486.0_f64) * t461 * t221 * t624 * t462;
    let t44799 = t1250 * t606;
    let t44800 = t2258 * t1248 * t44799;
    (t44789, t44792, t44797, t44800)
}
