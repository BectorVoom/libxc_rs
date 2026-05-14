//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 571/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk571<F: Float>(t1287: F, t487: F, t5284: F, t3781: F, t460: F, t1248: F, t3302: F, t471: F, t5332: F, t1811: F, t473: F, t1214: F, t489: F, t5412: F, t1204: F, t1234: F, t1281: F, t1285: F, t1288: F, t1291: F, t1770: F, t1818: F, t1822: F, t1825: F, t3666: F, t3670: F, t3746: F, t3755: F, t490: F, t5216: F, t5326: F, t5436: F, t5443: F, t5446: F, t5449: F, t5452: F, t5459: F, t5463: F, t5466: F, t5470: F) -> (F, F) {
    let t5474 = t487 * t5284 * t1287;
    let t5477 = t3781 * t487;
    let t5478 = t460 * t5477;
    let t5479 = t3302 * t1248;
    let t5480 = t5479 * t471;
    let t5481 = t5332 * t5480;
    let t5486 = t473 * t1811;
    let t5487 = t5486 * t1214;
    let t5491 = t1811 * t1248 * t1287;
    let t5494 = t489 * t5412;
    let t5497 = 0.65854491829355115987e0 * t5216 * t490 - 0.65854491829355115987e0 * t5326 * t1281 + 0.65854491829355115987e0 * t5436 * t1288 + 0.65854491829355115987e0 * t1770 * t1291 - 0.65854491829355115987e0 * t3666 * t1818 + 0.13170898365871023197e1 * t3670 * t5443 - 0.65854491829355115987e0 * t3755 * t5446 - 0.65854491829355115987e0 * t1234 * t5449 - 0.65854491829355115987e0 * t1234 * t5452 + 0.65854491829355115987e0 * t3746 * t1822 - 0.65854491829355115987e0 * t3755 * t5459 + 0.13170898365871023197e1 * t5463 * t5466 + 0.65854491829355115987e0 * t1285 * t5470 + 0.65854491829355115987e0 * t1285 * t5474 - 0.65854491829355115987e0 * t5478 * t5481 + 0.65854491829355115987e0 * t1204 * t1825 - 0.65854491829355115987e0 * t1234 * t5487 + 0.65854491829355115987e0 * t1285 * t5491 + 0.65854491829355115987e0 * t460 * t5494;
    (t5480, t5497)
}
