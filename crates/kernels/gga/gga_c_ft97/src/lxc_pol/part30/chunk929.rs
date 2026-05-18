//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 929/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk929<F: Float>(t1454: F, t6061: F, t7484: F, t771: F, t1424: F, t6192: F, t33243: F, t683: F, t1403: F, t33258: F, t681: F, t24237: F, t33504: F) -> (F, F, F, F, F, F) {
    let t140490 = t6061 * t1454;
    let t140495 = t7484 * t771;
    let t140508 = t1424 * t6192;
    let t140513 = t683 * t33243;
    let t140535 = t1403 * t681 * t33258;
    let t140556 = t24237 * t33504;
    (t140490, t140495, t140508, t140513, t140535, t140556)
}
