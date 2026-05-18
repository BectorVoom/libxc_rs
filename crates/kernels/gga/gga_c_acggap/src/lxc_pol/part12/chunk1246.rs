//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1246/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1246<F: Float>(t2226: F, t33802: F, t2131: F, t2132: F, t2385: F, t847: F, t2230: F, t33429: F, t8100: F, t8397: F, t8061: F, t8998: F) -> (F, F, F, F, F) {
    let t38471 = F::new(0.17347256376410398924e1) * t33802 * t2226;
    let t38474 = t2131 * t2132 * t2385 * t847;
    let t38481 = F::new(0.17347256376410398924e1) * t33429 * t2230;
    let t38487 = F::new(0.17347256376410398924e1) * t8397 * t8100;
    let t38489 = F::new(0.17347256376410398924e1) * t8998 * t8061;
    (t38471, t38474, t38481, t38487, t38489)
}
