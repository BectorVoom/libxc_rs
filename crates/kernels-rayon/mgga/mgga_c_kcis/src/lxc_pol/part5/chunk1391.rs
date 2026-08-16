//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1391/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1391(t21196: f64, t21199: f64, t21201: f64, t21203: f64, t21206: f64, t21209: f64, t21212: f64, t21215: f64, t21218: f64, t21221: f64, t21224: f64, t11455: f64, t12717: f64, t12718: f64, t21268: f64, t21270: f64, t21273: f64, t21275: f64, t21278: f64, t21281: f64, t21283: f64, t21286: f64) -> (f64, f64) {
    let t22956 = 0.17215833333333333333e0_f64 * t21196 - 0.104195e0_f64 * t21199 - 0.13892666666666666667e0_f64 * t21201 + 0.69463333333333333333e-1_f64 * t21203 - 0.41318e1_f64 * t21206 - 0.34431666666666666667e0_f64 * t21209 + 0.103295e1_f64 * t21212 + 0.13892666666666666667e0_f64 * t21215 - 0.62517e0_f64 * t21218 - 0.83356e0_f64 * t21221 + 0.20839e0_f64 * t21224;
    let t22977 = -0.11577222222222222222e0_f64 * t11455 - t12717 - t12718 + 0.3529725e1_f64 * t21268 + 0.6311625e0_f64 * t21270 + 0.264729375e1_f64 * t21273 - 0.3529725e1_f64 * t21275 - 0.17648625e1_f64 * t21278 - 0.157790625e0_f64 * t21281 + 0.6311625e0_f64 * t21283 + 0.31558125e0_f64 * t21286;
    (t22956, t22977)
}
