//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1629/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1629<F: Float>(t12772: F, t12846: F, t5331: F, t12776: F, t3625: F, t12780: F, t1121: F, t13045: F, t606: F, t13052: F, t13054: F, t3172: F) -> (F, F, F, F, F) {
    let t44711 = t5331 * t12772 * t12846;
    let t44726 = t3625 * t12772 * t12776;
    let t44729 = t3625 * t12772 * t12780;
    let t44737 = t13045 * t1121;
    let t44738 = t44737 * t606;
    let t44748 = t13052 * t3172 * t13054;
    (t44711, t44726, t44729, t44738, t44748)
}
