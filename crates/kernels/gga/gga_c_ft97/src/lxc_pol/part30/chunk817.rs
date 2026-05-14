//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 817/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk817<F: Float>(t2567: F, t6061: F, t2492: F, t6907: F, t1443: F, t2372: F, t2486: F, t6154: F, t1456: F, t9802: F, t6837: F, t761: F, t255: F, t41848: F, t28128: F, t53798: F) -> (F, F, F, F, F, F, F, F, F) {
    let t110019 = t2567 * t6061;
    let t110369 = t2492 * t6907;
    let t110401 = t2372 * t1443;
    let t110438 = t2486 * t6154;
    let t110478 = t2492 * t1456;
    let t110539 = t9802 * t1456;
    let t110629 = t761 * t6837;
    let t110660 = t41848 * t255;
    let t110669 = t53798 * t28128;
    (t110019, t110369, t110401, t110438, t110478, t110539, t110629, t110660, t110669)
}
