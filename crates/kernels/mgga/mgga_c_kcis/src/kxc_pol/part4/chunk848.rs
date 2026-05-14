//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 848/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk848<F: Float>(t127: F, t129: F, t130: F, t2379: F, t2496: F, t2507: F, t2508: F, t2514: F, t2518: F, t344: F, t60: F, t684: F, t756: F, t763: F, t764: F, t768: F, t8543: F, t8547: F, t8557: F, t8562: F, t8566: F, t8567: F, t8605: F, t8611: F, t8618: F, t8747: F) -> (F,) {
    let t8750 = -0.39422577999999999998e-2 * t8543 * t2508 - 0.59133866999999999997e-2 * t2507 * t8547 + 0.78845155999999999997e-2 * t127 * t756 * t2514 - 0.59133866999999999997e-2 * t763 * t764 * t2514 - 0.11826773399999999999e-1 * t127 * t129 * t8557 + 0.11826773399999999999e-1 * t344 * t8562 + 0.13140859333333333333e-2 * t8566 * t8567 - 0.19711288999999999999e-2 * t127 * t129 * t8605 - 12.0 * t684 * t2379 + 0.58403819259259259257e-3 * t127 * t8611 * t130 + 0.13140859333333333333e-2 * t127 * t2496 * t768 + 0.21901432222222222225e-3 * t763 * t8618 * t130 - 0.39422577999999999999e-2 * t127 * t756 * t2518 + 0.29566933499999999998e-2 * t763 * t764 * t2518 - 4.0 * t60 * t8747;
    (t8750,)
}
