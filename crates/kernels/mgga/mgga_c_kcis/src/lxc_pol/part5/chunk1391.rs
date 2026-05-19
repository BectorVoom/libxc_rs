//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1391/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1391<F: Float>(t21196: F, t21199: F, t21201: F, t21203: F, t21206: F, t21209: F, t21212: F, t21215: F, t21218: F, t21221: F, t21224: F, t11455: F, t12717: F, t12718: F, t21268: F, t21270: F, t21273: F, t21275: F, t21278: F, t21281: F, t21283: F, t21286: F) -> (F, F) {
    let t22956 = F::cast_from(0.17215833333333333333e0_f64) * t21196 - F::new(0.104195e0) * t21199 - F::cast_from(0.13892666666666666667e0_f64) * t21201 + F::cast_from(0.69463333333333333333e-1_f64) * t21203 - F::new(0.41318e1) * t21206 - F::cast_from(0.34431666666666666667e0_f64) * t21209 + F::new(0.103295e1) * t21212 + F::cast_from(0.13892666666666666667e0_f64) * t21215 - F::new(0.62517e0) * t21218 - F::new(0.83356e0) * t21221 + F::new(0.20839e0) * t21224;
    let t22977 = -F::cast_from(0.11577222222222222222e0_f64) * t11455 - t12717 - t12718 + F::new(0.3529725e1) * t21268 + F::new(0.6311625e0) * t21270 + F::cast_from(0.264729375e1_f64) * t21273 - F::new(0.3529725e1) * t21275 - F::new(0.17648625e1) * t21278 - F::cast_from(0.157790625e0_f64) * t21281 + F::new(0.6311625e0) * t21283 + F::new(0.31558125e0) * t21286;
    (t22956, t22977)
}
