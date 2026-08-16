//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 643/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk643(t4629: f64, t8919: f64, t1882: f64, t8522: f64, t706: f64, t4657: f64, t8500: f64, t1887: f64, t8536: f64, t2399: f64, t1421: f64, t456: f64, t4586: f64, t604: f64, t6998: f64, t7020: f64, t7043: f64, t8616: f64, t8896: f64, t8900: f64, t8904: f64, t8908: f64, t8912: f64, t8916: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8920 = t4629 * t8919;
    let t8923 = t1882 * t8522;
    let t8924 = t706 * t8923;
    let t8927 = t4657 * t8500;
    let t8928 = t706 * t8927;
    let t8931 = t1887 * t8536;
    let t8932 = t706 * t8931;
    let t8935 = t2399 * t2399;
    let t8939 = -t4586 + 0.8760572888888888889e-3_f64 * t6998 + 0.19711289e-2_f64 * t7020 - 0.13140859333333333333e-2_f64 * t7043 + 0.10950716111111111111e-2_f64 * t1421 * t8896 + 0.19711289e-2_f64 * t1421 * t8900 - 0.13140859333333333333e-2_f64 * t1421 * t8904 - 0.13140859333333333333e-2_f64 * t1421 * t8908 + 0.65704296666666666667e-3_f64 * t1421 * t8912 + 0.7391733375e-3_f64 * t456 * t8916 - 0.295669335e-2_f64 * t1421 * t8920 + 0.1478346675e-2_f64 * t456 * t8924 + 0.19711289e-2_f64 * t456 * t8928 - 0.98556445e-3_f64 * t456 * t8932 - 4.0_f64 * t8935 - 4.0_f64 * t604 * t8616;
    (t8920, t8923, t8924, t8927, t8928, t8931, t8932, t8939)
}
