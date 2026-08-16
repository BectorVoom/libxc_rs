//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2283/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2283<F: Float>(t47172: F, t708: F, t41295: F, t157: F, t41284: F, t12940: F, t12923: F, t12939: F, t2244: F, t12892: F, t12908: F, t2250: F, t4194: F) -> (F, F, F, F, F, F) {
    let t47174 = F::cast_from(12.0_f64) * t47172 * t708;
    let t47175 = F::cast_from(36.0_f64) * t41295;
    let t47176 = t41284 * t157;
    let t47178 = F::cast_from(72.0_f64) * t47176 * t12940;
    let t47180 = t12939 * t12923 * t2244;
    let t47181 = F::cast_from(72.0_f64) * t47180;
    let t47183 = F::cast_from(36.0_f64) * t12908 * t12892;
    let t47185 = t4194 * t12923 * t2250;
    (t47174, t47175, t47178, t47181, t47183, t47185)
}
