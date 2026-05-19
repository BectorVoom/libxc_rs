//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1136/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1136<F: Float>(t1445: F, t47187: F, t723: F, t813: F, t2536: F, t3720: F, t2009: F, t2021: F, t47294: F, t7572: F, t7573: F, t12252: F, t2628: F) -> (F, F, F, F) {
    let t47442 = F::cast_from(0.46011511144704899612e1_f64) * t813 * t1445 * t47187 * t723;
    let t47443 = t2536 * t3720;
    let t47445 = t2021 * t47443 * t2009;
    let t47448 = t7572 * t7573 * t47294;
    let t47450 = t12252 * t2628;
    (t47442, t47445, t47448, t47450)
}
