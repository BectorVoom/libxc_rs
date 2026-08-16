//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1220/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1220(t103586: f64, t125961: f64, t125984: f64, t126017: f64, t126030: f64, t127593: f64, t127596: f64, t1544: f64, t1940: f64, t2403: f64, t25445: f64, t26425: f64, t26585: f64, t26590: f64, t27375: f64, t28291: f64, t28460: f64, t32487: f64, t32491: f64, t32498: f64, t32505: f64, t34090: f64, t34097: f64, t4433: f64, t4541: f64, t7091: f64, t7432: f64, t8657: f64, t95976: f64) -> f64 {
    let t128060 = 2.0_f64 * t103586 * t1940 * t32505 + 2.0_f64 * t125961 * t1940 * t26590 - 3.0_f64 * t125984 * t2403 * t7432 + 2.0_f64 * t126017 * t1940 * t26590 - 3.0_f64 * t126030 * t2403 * t7432 + 2.0_f64 * t127593 * t1940 * t26590 - 3.0_f64 * t127596 * t2403 * t7432 + 3.0_f64 * t1544 * t2403 * t32487 + 2.0_f64 * t1940 * t34097 * t95976 - 3.0_f64 * t2403 * t26585 * t34090 - 3.0_f64 * t2403 * t27375 * t32491 - 3.0_f64 * t2403 * t28460 * t32498 + 6.0_f64 * t25445 * t26425 * t27375 - 6.0_f64 * t28291 * t4433 * t7091 + 6.0_f64 * t4433 * t4541 * t8657;
    t128060
}
