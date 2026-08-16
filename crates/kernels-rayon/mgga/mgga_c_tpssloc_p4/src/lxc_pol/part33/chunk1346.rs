//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1346/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1346(t1945: f64, t21390: f64, t5872: f64, t7593: f64, t1615: f64, t5392: f64, t6800: f64, t100163: f64, t100240: f64, t1058: f64, t1060: f64, t11046: f64, t11048: f64, t11065: f64, t11066: f64, t1539: f64, t1599: f64, t18086: f64, t1948: f64, t23601: f64, t23604: f64, t23633: f64, t25516: f64, t28666: f64, t3186: f64, t3188: f64, t381: f64, t5681: f64, t5836: f64, t5866: f64, t6687: f64, t6784: f64, t7620: f64, t83233: f64, t83245: f64, t89044: f64) -> (f64, f64, f64, f64) {
    let t106045 = t1945 * t21390;
    let t106058 = t7593 * t5872;
    let t106073 = t5392 * t1615 * t6800;
    let t106083 = -6.0_f64 * t11065 * t106045 * t11066 - 0.16449340668482264365e-1_f64 * t6687 * t6784 * t25516 * t5681 - 0.24674011002723396548e-1_f64 * t6687 * t1599 * t1948 * t381 * t5836 + 6.0_f64 * t3186 * t106058 * t3188 + 3.0_f64 * t1058 * t7593 * t5866 * t1060 + t11046 * t106045 * t11048 - 0.49348022005446793095e-1_f64 * t23601 * t89044 * t28666 - 0.16449340668482264365e-1_f64 * t100163 - 0.16449340668482264365e-1_f64 * t23633 * t83233 * t106073 - 0.82246703342411321826e-2_f64 * t83245 * t100240 * t23604 * t1539 + 3.0_f64 * t18086 * t7620;
    (t106045, t106058, t106073, t106083)
}
