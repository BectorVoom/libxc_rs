//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2201/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2201(t1625: f64, t23592: f64, t225: f64, t25791: f64, t23384: f64, t25413: f64, t1598: f64, t3014: f64, t1921: f64, t7577: f64, t25403: f64, t1066: f64, t14658: f64, t1599: f64, t23327: f64, t23332: f64, t23365: f64, t23594: f64, t23722: f64, t25424: f64, t25784: f64, t25797: f64, t25826: f64, t3010: f64, t4660: f64, t6687: f64, t6704: f64, t6705: f64, t7553: f64, t82400: f64, t82417: f64, t82426: f64, t83424: f64, t83453: f64) -> (f64, f64) {
    let t88138 = t23592 * t1625;
    let t88145 = t25791 * t225;
    let t88152 = 0.54831135561607547884e-2_f64 * t23384 * t25413;
    let t88155 = t1598 * t3014;
    let t88162 = t7577 * t1921;
    let t88167 = 0.54831135561607547884e-2_f64 * t23384 * t25403;
    let t88179 = 0.36554090374405031923e-2_f64 * t6687 * t88138 * t23594 - 0.16449340668482264365e-1_f64 * t6687 * t23365 * t25826 - 2.0_f64 * t88145 * t1066 + 0.27415567780803773942e-2_f64 * t6687 * t83424 * t7553 - t88152 + 0.54831135561607547884e-2_f64 * t82400 - t4660 * t23722 - 0.82246703342411321825e-2_f64 * t6687 * t88155 * t25797 + 0.82246703342411321825e-2_f64 * t6687 * t1599 * t83453 + 0.54831135561607547884e-2_f64 * t23327 * t88162 * t23332 - t88167 - 0.10966227112321509577e-1_f64 * t23327 * t82417 * t25424 - 0.82246703342411321825e-2_f64 * t6687 * t6704 * t6705 * t14658 + 0.91385225936012579807e-3_f64 * t82426 + 0.82246703342411321825e-2_f64 * t6687 * t3010 * t25784;
    (t88155, t88179)
}
