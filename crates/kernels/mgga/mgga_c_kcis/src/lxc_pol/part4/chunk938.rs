//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 938/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk938<F: Float>(t1658: F, t3703: F, t233: F, t1877: F, t2794: F, t5398: F, t915: F, t4538: F, t911: F, t4535: F, t1300: F, t6260: F, t446: F, t13003: F, t1646: F, t167: F, t2629: F) -> (F, F, F, F, F, F, F, F) {
    let t13047 = t1658 * t3703;
    let t13048 = t233 * t13047;
    let t13050 = t2794 * t1877;
    let t13052 = t915 * t5398;
    let t13053 = t233 * t13052;
    let t13055 = t911 * t4538;
    let t13057 = t911 * t4535;
    let t13059 = t1300 * t6260;
    let t13060 = t446 * t13059;
    let t13062 = t13003 * t1646;
    let t13065 = t2629 * t167;
    (t13048, t13050, t13053, t13055, t13057, t13060, t13062, t13065)
}
