//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 918/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk918<F: Float>(t28097: F, t761: F, t10052: F, t676: F, t24412: F, t737: F, t762: F, t9707: F, t2567: F, t6061: F, t2492: F, t6907: F) -> (F, F, F, F, F, F) {
    let t109755 = t28097 * t761;
    let t109848 = t676 * t10052;
    let t109926 = t737 * t24412;
    let t110010 = t9707 * t762;
    let t110019 = t2567 * t6061;
    let t110369 = t2492 * t6907;
    (t109755, t109848, t109926, t110010, t110019, t110369)
}
