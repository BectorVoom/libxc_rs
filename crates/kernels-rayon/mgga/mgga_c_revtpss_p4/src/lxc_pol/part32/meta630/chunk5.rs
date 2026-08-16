//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2036/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2036(t102854: f64, t103586: f64, t105923: f64, t106561: f64, t106625: f64, t110177: f64, t1544: f64, t1583: f64, t18392: f64, t18875: f64, t1940: f64, t2071: f64, t2403: f64, t26585: f64, t26590: f64, t27384: f64, t28456: f64, t28460: f64, t29598: f64, t30439: f64, t4343: f64, t4433: f64, t4537: f64, t4541: f64, t50080: f64, t5966: f64, t6079: f64, t7428: f64, t7432: f64, t8020: f64, t890: f64, t95976: f64) -> f64 {
    let t110839 = 6.0_f64 * t2403 * t26590 * t106561 + 3.0_f64 * t2403 * t2071 * t18392 + 4.0_f64 * t1940 * t103586 * t27384 - 6.0_f64 * t2403 * t26585 * t29598 - 6.0_f64 * t2403 * t28460 * t18875 + 6.0_f64 * t2403 * t28456 * t1544 - 6.0_f64 * t2403 * t7432 * t106625 - 2.0_f64 * t1940 * t28460 * t4537 + 12.0_f64 * t4541 * t8020 * t4433 - 3.0_f64 * t2403 * t7432 * t105923 - 2.0_f64 * t1940 * t102854 * t1583 - t1940 * t110177 * t890 + 6.0_f64 * t4541 * t7428 * t5966 + 2.0_f64 * t1940 * t95976 * t6079 + 6.0_f64 * t2403 * t8020 * t4343 + 6.0_f64 * t50080 * t30439;
    t110839
}
