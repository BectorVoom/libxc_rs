//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1005/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1005<F: Float>(t1980: F, t35500: F, t7476: F, t1988: F, t8486: F, t1967: F, t8838: F, t4360: F, t7741: F, t13287: F, t31057: F, t33953: F, t5122: F) -> (F, F, F, F, F) {
    let t35502 = t1980 * t7476 * t35500;
    let t35503 = F::cast_from(0.7145669686344956162e-3_f64) * t35502;
    let t35513 = t1988 * t8486;
    let t35514 = F::cast_from(0.94344276868812456204e-3_f64) * t35513;
    let t35515 = t1967 * t8838;
    let t35529 = t7741 * t4360;
    let t35549 = t31057 * t13287 * t33953 * t5122;
    (t35503, t35514, t35515, t35529, t35549)
}
