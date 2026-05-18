//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1040/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1040<F: Float>(t31081: F, t3564: F, t1428: F, t30605: F, t457: F, t12872: F, t30892: F, t1421: F, t26710: F, t26712: F, t30738: F, t31063: F, t31067: F, t31071: F, t31075: F, t31078: F, t338: F, t456: F) -> (F, F, F) {
    let t31082 = t3564 * t31081;
    let t31089 = t1428 * t30605;
    let t31090 = t457 * t31089;
    let t31093 = t12872 * t30892;
    let t31094 = t457 * t31093;
    let t31097 = F::new(0.295669335e-2) * t1421 * t31063 + F::new(0.295669335e-2) * t1421 * t31067 - F::new(0.19711289e-2) * t1421 * t31071 - F::new(0.19711289e-2) * t1421 * t31075 - F::new(0.39422577999999999999e-2) * t1421 * t31078 + F::new(0.887008005e-2) * t1421 * t31082 + F::new(0.39422577999999999999e-2) * t26710 + F::new(0.295669335e-2) * t26712 - F::new(4.0) * t338 * t30738 + F::new(0.1478346675e-2) * t456 * t31090 - F::new(0.59133867e-2) * t456 * t31094;
    (t31089, t31093, t31097)
}
