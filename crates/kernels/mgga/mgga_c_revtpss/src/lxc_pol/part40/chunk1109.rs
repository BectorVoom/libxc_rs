//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1109/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1109<F: Float>(t2615: F, t4311: F, t10588: F, t10577: F, t10582: F, t10584: F, t10586: F, t10592: F, t11084: F, t14385: F, t14388: F, t14392: F, t14396: F, t14397: F, t14428: F, t1544: F, t1940: F, t2394: F, t2403: F, t4541: F, t4546: F, t890: F, t9514: F, t9517: F, t9521: F, t9524: F) -> (F, F, F) {
    let t14433 = 8.0 * t4311 * t2615;
    let t14434 = 0.5848223622634646207e0 * t10588;
    let t14435 = -3.0 * t11084 * t1544 * t2403 - 2.0 * t14397 * t1940 * t890 + 6.0 * t2394 * t4541 * t4546 + t10577 + t10582 - t10584 - t10586 + t10592 + t14385 + t14388 + t14392 + t14396 + t14428 + t14433 - t14434 + t9514 - t9517 - t9521 - t9524;
    (t14433, t14434, t14435)
}
