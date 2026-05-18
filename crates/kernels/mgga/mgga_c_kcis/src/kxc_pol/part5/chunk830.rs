//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 830/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk830<F: Float>(t1154: F, t3410: F, t6272: F, t1155: F, t6276: F, t304: F, t6478: F, t1153: F, t1757: F, t1761: F, t1780: F, t3381: F, t3392: F, t348: F, t365: F, t368: F, t5130: F, t5151: F, t6589: F, t6593: F, t6597: F, t6601: F, t6605: F, t6641: F, t6661: F, t6665: F, t86: F) -> (F, F, F, F) {
    let t6669 = t1154 * t3410 * t6272;
    let t6673 = t1154 * t1155 * t6276;
    let t6676 = t304 * t6478;
    let t6680 = F::new(0.619125e-2) * t6641 * t348 + F::new(0.1857375e-1) * t1780 * t1757 - F::new(0.123825e-1) * t1780 * t1761 + F::new(0.46434375e-2) * t365 * t6589 - F::new(0.1857375e-1) * t3381 * t6593 + F::new(0.9286875e-2) * t365 * t6597 + F::new(0.123825e-1) * t365 * t6601 - F::new(0.619125e-2) * t365 * t6605 + t3392 - F::new(0.35374814814814814814e-1) * t5130 - F::new(0.53062222222222222222e-1) * t5151 - F::new(0.44218518518518518518e-1) * t1153 * t6661 - F::new(0.53062222222222222222e-1) * t1153 * t6665 + F::new(0.53062222222222222222e-1) * t1153 * t6669 - F::new(0.26531111111111111111e-1) * t1153 * t6673 - F::new(0.39796666666666666666e-1) * t86 * t368 * t6676;
    (t6669, t6673, t6676, t6680)
}
