//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1257/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1257<F: Float>(t19815: F, t42530: F, t7718: F, t1020: F, t27836: F, t4796: F, t1087: F, t303: F, t6497: F, t100162: F, t100170: F, t27077: F, t28184: F, t28190: F, t92896: F, t92898: F, t97105: F, t97106: F, t97153: F) -> (F, F, F, F) {
    let t100540 = t42530 * t7718 * t19815;
    let t100547 = t1020 * t27836 * t4796;
    let t100553 = t303 * t1087 * t6497;
    let t100555 = -t97105 - F::new(0.30918233506944444445e-4) * t97106 - F::new(0.7722800925925925926e-4) * t92896 - F::new(0.7722800925925925926e-4) * t92898 + F::new(0.77382407407407407408e-2) * t100540 - F::new(0.92835860883789062501e-5) * t27077 * t100170 - F::new(0.69505208333333333334e-3) * t28190 * t28184 + F::new(0.15476481481481481481e-2) * t100547 + F::new(0.557015165302734375e-4) * t27077 * t100162 + F::new(0.51485339506172839507e-4) * t97153 - F::new(0.38691203703703703703e-3) * t100553;
    (t100540, t100547, t100553, t100555)
}
