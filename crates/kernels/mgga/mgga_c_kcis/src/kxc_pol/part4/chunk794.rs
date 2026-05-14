//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 794/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk794<F: Float>(t1919: F, t3918: F, t1578: F, t3795: F, t3881: F, t3926: F, t3933: F, t5469: F, t5472: F, t5475: F, t5479: F, t5514: F, t5516: F, t5557: F, t5559: F, t5562: F, t5565: F, t5568: F, t5571: F) -> (F, F, F) {
    let t5595 = t3918 * t1919;
    let t5596 = t5595 * t1578;
    let t5613 = -0.1294625e1 * t5514 + 0.258925e1 * t5516 + t3926 + 0.10064166666666666667e0 * t3795 + 0.10064166666666666667e0 * t5469 - 0.20128333333333333333e0 * t5472 + 0.60385e0 * t5475 + 0.60385e0 * t5479 + 0.82524375e-1 * t5557 + 0.16504875e0 * t5559 + t3933 + 0.5519e-1 * t3881 + 0.5519e-1 * t5562 - 0.27595e-1 * t5565 + 0.16557e0 * t5568 + 0.16557e0 * t5571;
    (t5595, t5596, t5613)
}
