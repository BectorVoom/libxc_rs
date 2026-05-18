//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1133/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1133<F: Float>(t12857: F, t1588: F, t12856: F, t609: F, t625: F, t4313: F, t4390: F, t4413: F, t4473: F, t4479: F, t12938: F, t629: F, t632: F) -> (F, F, F, F, F, F) {
    let t40484 = t1588 * t12857;
    let t40512 = t609 / t12856 / t625;
    let t40514 = t4313 * t4313;
    let t40515 = F::new(1.0) / t40514;
    let t40541 = t4390 * t4413;
    let t40556 = t4473 * t4479;
    let t40653 = t629 / t12938 / t632;
    (t40484, t40512, t40515, t40541, t40556, t40653)
}
