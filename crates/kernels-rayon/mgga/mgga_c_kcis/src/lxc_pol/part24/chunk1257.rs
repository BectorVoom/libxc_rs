//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1257/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1257(t19815: f64, t42530: f64, t7718: f64, t1020: f64, t27836: f64, t4796: f64, t1087: f64, t303: f64, t6497: f64, t100162: f64, t100170: f64, t27077: f64, t28184: f64, t28190: f64, t92896: f64, t92898: f64, t97105: f64, t97106: f64, t97153: f64) -> (f64, f64, f64, f64) {
    let t100540 = t42530 * t7718 * t19815;
    let t100547 = t1020 * t27836 * t4796;
    let t100553 = t303 * t1087 * t6497;
    let t100555 = -t97105 - 0.30918233506944444445e-4_f64 * t97106 - 0.7722800925925925926e-4_f64 * t92896 - 0.7722800925925925926e-4_f64 * t92898 + 0.77382407407407407408e-2_f64 * t100540 - 0.92835860883789062501e-5_f64 * t27077 * t100170 - 0.69505208333333333334e-3_f64 * t28190 * t28184 + 0.15476481481481481481e-2_f64 * t100547 + 0.557015165302734375e-4_f64 * t27077 * t100162 + 0.51485339506172839507e-4_f64 * t97153 - 0.38691203703703703703e-3_f64 * t100553;
    (t100540, t100547, t100553, t100555)
}
