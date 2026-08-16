//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 793/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk793(t1022: f64, t3131: f64, t4593: f64, t4582: f64, t1023: f64, t135: f64, t1606: f64, t973: f64, t3966: f64, t998: f64, t974: f64, t1041: f64, t1607: f64, t1622: f64, t2960: f64, t3039: f64, t3048: f64, t3054: f64, t3070: f64, t3084: f64, t3092: f64, t3130: f64, t4562: f64, t4565: f64, t4572: f64, t4575: f64, t4579: f64, t4585: f64, t4590: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4594 = t3131 * t1022;
    let t4595 = t4593 * t4594;
    let t4596 = t4582 * t4595;
    let t4599 = t4593 * t1023;
    let t4600 = t4582 * t4599;
    let t4603 = t135 * t1606;
    let t4604 = t973 * t4603;
    let t4608 = t998 * t3966;
    let t4609 = t974 * t4608;
    let t4613 = t3054 / 6912.0_f64 - t973 * t4562 / 144.0_f64 + t973 * t4565 / 216.0_f64 - t3048 * t1622 / 864.0_f64 + t4572 / 6912.0_f64 + t3070 * t4575 / 4608.0_f64 + t3070 * t4579 / 4608.0_f64 - t1041 * t4585 / 2304.0_f64 + 5.0_f64 / 13824.0_f64 * t1041 * t4590 + t3130 * t4596 / 1536.0_f64 - t3039 * t4600 / 3072.0_f64 + t4604 / 864.0_f64 - t2960 * t1607 / 108.0_f64 + t973 * t4609 / 288.0_f64 - t3084 - t3092 / 864.0_f64;
    (t4594, t4595, t4596, t4599, t4600, t4603, t4608, t4609, t4613)
}
