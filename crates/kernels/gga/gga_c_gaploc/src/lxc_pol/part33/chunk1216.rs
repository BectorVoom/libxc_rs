//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1216/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1216<F: Float>(t10704: F, t1850: F, t10636: F, t5227: F, t1841: F, t3487: F, t7275: F, t734: F, t10826: F, t2536: F, t1944: F, t3444: F) -> (F, F, F, F, F) {
    let t32622 = t1850 * t10704;
    let t32623 = F::new(0.85450291446024714264e-3) * t32622;
    let t32625 = F::new(0.17090058289204942853e-2) * t5227 * t10636;
    let t32629 = F::new(0.17090058289204942853e-2) * t1841 * t7275 * t3487 * t734;
    let t32633 = F::new(0.17090058289204942853e-2) * t1841 * t2536 * t10826 * t734;
    let t32634 = t1944 * t3444;
    (t32623, t32625, t32629, t32633, t32634)
}
