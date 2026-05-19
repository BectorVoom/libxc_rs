//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 643/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk643<F: Float>(t4629: F, t8919: F, t1882: F, t8522: F, t706: F, t4657: F, t8500: F, t1887: F, t8536: F, t2399: F, t1421: F, t456: F, t4586: F, t604: F, t6998: F, t7020: F, t7043: F, t8616: F, t8896: F, t8900: F, t8904: F, t8908: F, t8912: F, t8916: F) -> (F, F, F, F, F, F, F, F) {
    let t8920 = t4629 * t8919;
    let t8923 = t1882 * t8522;
    let t8924 = t706 * t8923;
    let t8927 = t4657 * t8500;
    let t8928 = t706 * t8927;
    let t8931 = t1887 * t8536;
    let t8932 = t706 * t8931;
    let t8935 = t2399 * t2399;
    let t8939 = -t4586 + F::cast_from(0.8760572888888888889e-3_f64) * t6998 + F::new(0.19711289e-2) * t7020 - F::cast_from(0.13140859333333333333e-2_f64) * t7043 + F::cast_from(0.10950716111111111111e-2_f64) * t1421 * t8896 + F::new(0.19711289e-2) * t1421 * t8900 - F::cast_from(0.13140859333333333333e-2_f64) * t1421 * t8904 - F::cast_from(0.13140859333333333333e-2_f64) * t1421 * t8908 + F::cast_from(0.65704296666666666667e-3_f64) * t1421 * t8912 + F::cast_from(0.7391733375e-3_f64) * t456 * t8916 - F::cast_from(0.295669335e-2_f64) * t1421 * t8920 + F::cast_from(0.1478346675e-2_f64) * t456 * t8924 + F::new(0.19711289e-2) * t456 * t8928 - F::new(0.98556445e-3) * t456 * t8932 - F::new(4.0) * t8935 - F::new(4.0) * t604 * t8616;
    (t8920, t8923, t8924, t8927, t8928, t8931, t8932, t8939)
}
