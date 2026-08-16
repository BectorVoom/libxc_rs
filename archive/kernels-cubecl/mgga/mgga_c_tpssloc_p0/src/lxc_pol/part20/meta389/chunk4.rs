//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1766/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1766<F: Float>(t2: F, t873: F, t584: F, t265: F, t16: F, t4331: F, t10723: F, t4496: F, t959: F, t2944: F, t4483: F, t2940: F, t4493: F) -> (F, F, F, F, F, F, F, F) {
    let t13501 = t873 * t2;
    let t13503 = F::cast_from(2.0_f64) * t13501 * t584;
    let t13504 = t265 * t584;
    let t13506 = F::cast_from(3.0_f64) * t4331 * t16;
    let t13508 = t4496 * t10723;
    let t13510 = F::cast_from(0.17315859105681463759e2_f64) * t959 * t13508;
    let t13512 = F::cast_from(0.11696447245269292414e1_f64) * t4483 * t2944;
    let t13514 = F::cast_from(0.11696447245269292414e1_f64) * t2940 * t4493;
    (t13501, t13503, t13504, t13506, t13508, t13510, t13512, t13514)
}
