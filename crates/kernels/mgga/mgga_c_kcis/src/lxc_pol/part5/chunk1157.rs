//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1157/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1157<F: Float>(t21400: F, t469: F, t21196: F, t21199: F, t21201: F, t21203: F, t21206: F, t21209: F, t21212: F, t21215: F, t21218: F, t21221: F, t21224: F, t11455: F, t11479: F, t11482: F, t21268: F, t21270: F, t21273: F, t21275: F, t21278: F, t21281: F, t21283: F, t21286: F) -> (F, F, F) {
    let t21402 = 0.62182e-1 * t21400 * t469;
    let t21424 = 0.10064166666666666667e0 * t21196 - 0.82785e-1 * t21199 - 0.11038e0 * t21201 + 0.5519e-1 * t21203 - 0.24154e1 * t21206 - 0.20128333333333333333e0 * t21209 + 0.60385e0 * t21212 + 0.11038e0 * t21215 - 0.49671e0 * t21218 - 0.66228e0 * t21221 + 0.16557e0 * t21224;
    let t21445 = -0.91983333333333333333e-1 * t11455 - t11479 - t11482 + 0.258925e1 * t21268 + 0.16504875e0 * t21270 + 0.19419375e1 * t21273 - 0.258925e1 * t21275 - 0.1294625e1 * t21278 - 0.412621875e-1 * t21281 + 0.16504875e0 * t21283 + 0.82524375e-1 * t21286;
    (t21402, t21424, t21445)
}
