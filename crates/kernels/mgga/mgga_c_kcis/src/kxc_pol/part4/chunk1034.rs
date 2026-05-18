//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1034/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1034<F: Float>(t2540: F, t2534: F, t4543: F, t911: F, t1658: F, t3703: F, t233: F, t1877: F, t2794: F, t5398: F, t915: F, t4538: F) -> (F, F, F, F, F, F, F) {
    let t13043 = F::new(6.0) * t2540;
    let t13044 = F::new(6.0) * t2534;
    let t13045 = t911 * t4543;
    let t13047 = t1658 * t3703;
    let t13048 = t233 * t13047;
    let t13050 = t2794 * t1877;
    let t13052 = t915 * t5398;
    let t13053 = t233 * t13052;
    let t13055 = t911 * t4538;
    (t13043, t13044, t13045, t13048, t13050, t13053, t13055)
}
