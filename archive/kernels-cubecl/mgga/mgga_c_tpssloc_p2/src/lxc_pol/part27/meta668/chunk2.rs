//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2357/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2357<F: Float>(t26502: F, t532: F, t1983: F, t6879: F, t2314: F, t26142: F, t4034: F, t1266: F, t26135: F, t652: F, t24987: F, t6997: F) -> (F, F, F, F, F) {
    let t91620 = t532 * t26502;
    let t91623 = F::cast_from(6.0_f64) * t1983 * t91620 * t6879;
    let t91625 = F::cast_from(4.0_f64) * t2314 * t26142;
    let t91627 = F::cast_from(4.0_f64) * t4034 * t26142;
    let t91630 = F::cast_from(4.0_f64) * t652 * t1266 * t26135;
    let t91637 = F::cast_from(2.0_f64) * t24987 * t6997;
    (t91623, t91625, t91627, t91630, t91637)
}
