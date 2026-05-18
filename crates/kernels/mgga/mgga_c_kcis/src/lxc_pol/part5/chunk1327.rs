//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1327/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1327<F: Float>(t21936: F, t5661: F, t17009: F, t20984: F, t4170: F, t16693: F, t16692: F, t21655: F, t5662: F, t16788: F, t16793: F, t21453: F, t21919: F, t21923: F, t21926: F, t21929: F, t21933: F, t507: F) -> (F, F, F, F, F) {
    let t21937 = t5661 * t21936;
    let t21939 = t17009 * t20984;
    let t21940 = t4170 * t21939;
    let t21941 = t5661 * t21940;
    let t21943 = t16693 * t20984;
    let t21944 = t4170 * t21943;
    let t21945 = t16692 * t21944;
    let t21947 = t5662 * t21655;
    let t21948 = t4170 * t21947;
    let t21949 = t16788 * t21948;
    let t21951 = t21453 * t507 + F::new(0.33163888888888888888e-2) * t21919 - F::new(0.22109259259259259259e-2) * t16793 + F::new(0.88437037037037037034e-2) * t21923 - F::new(0.58958024691358024689e-2) * t21926 + F::new(0.17687407407407407407e-1) * t21929 - F::new(0.27636574074074074073e-2) * t21933 + F::new(0.18424382716049382715e-2) * t21937 - F::new(0.16581944444444444444e-1) * t21941 + F::new(0.73697530864197530861e-2) * t21945 - F::new(0.11054629629629629629e-1) * t21949;
    (t21937, t21941, t21945, t21949, t21951)
}
