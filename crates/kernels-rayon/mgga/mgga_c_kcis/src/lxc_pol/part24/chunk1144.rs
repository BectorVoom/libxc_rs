//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1144/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1144(t2155: f64, t92016: f64, t91967: f64, t91973: f64, t91975: f64, t91979: f64, t91983: f64, t91987: f64, t91989: f64, t91992: f64, t91994: f64, t91996: f64, t91999: f64, t92002: f64, t92005: f64, t92007: f64, t92010: f64, t92012: f64) -> f64 {
    let t92017 = t2155 * t92016;
    let t92019 = 0.41703125000000000001e-2_f64 * t91967 - 0.41703125000000000001e-2_f64 * t91973 + 0.83479230468750000001e-3_f64 * t91975 - 0.208515625e-2_f64 * t91979 - 0.2782641015625e-3_f64 * t91983 + 0.12985658072916666667e-2_f64 * t91987 - 0.208515625e-2_f64 * t91989 + 0.97307291666666666666e-2_f64 * t91992 - 0.2782641015625e-3_f64 * t91994 - 0.8347923046875e-3_f64 * t91996 + 0.23425829475308641975e-1_f64 * t91999 - 0.16217881944444444444e-1_f64 * t92002 + 0.48653645833333333332e-2_f64 * t92005 + 0.208515625e-2_f64 * t92007 + 0.2782641015625e-3_f64 * t92010 + 0.97307291666666666666e-2_f64 * t92012 + 0.41703125000000000001e-2_f64 * t92017;
    t92019
}
