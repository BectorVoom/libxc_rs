//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1241/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1241<F: Float>(t33796: F, t8313: F, t33799: F, t8310: F, t38086: F, t4210: F, t7942: F, t524: F, t9427: F, t36429: F, t7963: F, t4241: F) -> (F, F, F, F, F, F) {
    let t38377 = F::new(0.17347256376410398924e1) * t33796 * t8313;
    let t38379 = F::new(0.17347256376410398924e1) * t33799 * t8310;
    let t38382 = F::new(0.17347256376410398924e1) * t7942 * t38086 * t4210;
    let t38383 = t9427 * t524;
    let t38386 = F::new(0.34694512752820797848e1) * t7963 * t38383 * t36429;
    let t38389 = F::new(0.34694512752820797848e1) * t7942 * t38383 * t4241;
    (t38377, t38379, t38382, t38383, t38386, t38389)
}
