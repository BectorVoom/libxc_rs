//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 960/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk960(t5332: f64, t5480: f64, t1811: f64, t473: f64, t1214: f64, t1248: f64, t1287: f64, t489: f64, t5412: f64, t1204: f64, t1234: f64, t1281: f64, t1285: f64, t1288: f64, t1291: f64, t1770: f64, t1818: f64, t1822: f64, t1825: f64, t3666: f64, t3670: f64, t3746: f64, t3755: f64, t460: f64, t490: f64, t5216: f64, t5326: f64, t5436: f64, t5443: f64, t5446: f64, t5449: f64, t5452: f64, t5459: f64, t5463: f64, t5466: f64, t5470: f64, t5474: f64, t5478: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5481 = t5332 * t5480;
    let t5486 = t473 * t1811;
    let t5487 = t5486 * t1214;
    let t5491 = t1811 * t1248 * t1287;
    let t5494 = t489 * t5412;
    let t5497 = 0.65854491829355115987e0_f64 * t5216 * t490 - 0.65854491829355115987e0_f64 * t5326 * t1281 + 0.65854491829355115987e0_f64 * t5436 * t1288 + 0.65854491829355115987e0_f64 * t1770 * t1291 - 0.65854491829355115987e0_f64 * t3666 * t1818 + 0.13170898365871023197e1_f64 * t3670 * t5443 - 0.65854491829355115987e0_f64 * t3755 * t5446 - 0.65854491829355115987e0_f64 * t1234 * t5449 - 0.65854491829355115987e0_f64 * t1234 * t5452 + 0.65854491829355115987e0_f64 * t3746 * t1822 - 0.65854491829355115987e0_f64 * t3755 * t5459 + 0.13170898365871023197e1_f64 * t5463 * t5466 + 0.65854491829355115987e0_f64 * t1285 * t5470 + 0.65854491829355115987e0_f64 * t1285 * t5474 - 0.65854491829355115987e0_f64 * t5478 * t5481 + 0.65854491829355115987e0_f64 * t1204 * t1825 - 0.65854491829355115987e0_f64 * t1234 * t5487 + 0.65854491829355115987e0_f64 * t1285 * t5491 + 0.65854491829355115987e0_f64 * t460 * t5494;
    (t5481, t5486, t5487, t5491, t5494, t5497)
}
