//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1196/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1196(t16236: f64, t5641: f64, t721: f64, t4795: f64, t5645: f64, t5651: f64, t13728: f64, t5656: f64, t3111: f64, t5660: f64, t1072: f64, t13703: f64, t1713: f64, t3126: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21737 = t16236 * t5641 * t721;
    let t21740 = t4795 * t5645 * t721;
    let t21743 = t4795 * t5651 * t721;
    let t21745 = t13728 * t5656;
    let t21747 = t3111 * t5660;
    let t21751 = t13703 * t1072 * t1713 * t3126;
    (t21737, t21740, t21743, t21745, t21747, t21751)
}
