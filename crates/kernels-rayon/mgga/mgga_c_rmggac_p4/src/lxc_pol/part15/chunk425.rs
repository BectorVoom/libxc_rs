//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 425/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk425(t1054: f64, t1089: f64, t1092: f64, t245: f64, t977: f64, t214: f64, t410: f64, t1038: f64, t1060: f64, t1061: f64, t1063: f64, t1072: f64, t1078: f64, t1116: f64, t1121: f64, t1124: f64, t1127: f64, t1128: f64, t167: f64, t180: f64, t403: f64, t4101: f64, t4103: f64, t4106: f64, t418: f64, t4190: f64, t4202: f64, t4203: f64, t4214: f64, t4220: f64, t4222: f64, t5: f64) -> (f64, f64) {
    let t4232 = 0.10685e0_f64 * t1054 * t245 * t1089 * t1092;
    let t4233 = t245 * t977;
    let t4237 = t214 * t410;
    let t4244 = t245 * t1038;
    let t4248 = t4101 - t4106 + 0.51947577317044391277e2_f64 * t1127 * t4190 - 6.0_f64 * t1061 * t403 * t1072 + 0.16562821945185185185e-2_f64 * t5 * t4103 * t167 + 0.56968947174242584612e-3_f64 * t5 * t4103 * t180 + 0.10254018858216406658e4_f64 * t4202 * t4203 - t4214 + t4220 + 6.0_f64 * t1078 * t4222 + 0.10274e0_f64 * t1054 * t245 * t1060 * t1063 - t4232 + 0.32530743900905219526e-1_f64 * t1054 * t4233 * t1121 + 0.21687162600603479684e-1_f64 * t1054 * t4237 * t418 - 0.16265371950452609763e-1_f64 * t1054 * t1116 * t1124 - 0.48159733137676571078e0_f64 * t1054 * t4244 * t1128;
    (t4232, t4248)
}
