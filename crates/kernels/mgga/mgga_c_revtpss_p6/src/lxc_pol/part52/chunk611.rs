//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 611/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk611<F: Float>(t1204: F, t1234: F, t1281: F, t1285: F, t1288: F, t1291: F, t1770: F, t1818: F, t1822: F, t1825: F, t3666: F, t3670: F, t3746: F, t3755: F, t460: F, t490: F, t5216: F, t5326: F, t5436: F, t5443: F, t5446: F, t5449: F, t5452: F, t5459: F, t5463: F, t5466: F, t5470: F, t5474: F, t5478: F, t5481: F, t5487: F, t5491: F, t5494: F) -> F {
    let t5497 = F::cast_from(0.65854491829355115987e0_f64) * t5216 * t490 - F::cast_from(0.65854491829355115987e0_f64) * t5326 * t1281 + F::cast_from(0.65854491829355115987e0_f64) * t5436 * t1288 + F::cast_from(0.65854491829355115987e0_f64) * t1770 * t1291 - F::cast_from(0.65854491829355115987e0_f64) * t3666 * t1818 + F::cast_from(0.13170898365871023197e1_f64) * t3670 * t5443 - F::cast_from(0.65854491829355115987e0_f64) * t3755 * t5446 - F::cast_from(0.65854491829355115987e0_f64) * t1234 * t5449 - F::cast_from(0.65854491829355115987e0_f64) * t1234 * t5452 + F::cast_from(0.65854491829355115987e0_f64) * t3746 * t1822 - F::cast_from(0.65854491829355115987e0_f64) * t3755 * t5459 + F::cast_from(0.13170898365871023197e1_f64) * t5463 * t5466 + F::cast_from(0.65854491829355115987e0_f64) * t1285 * t5470 + F::cast_from(0.65854491829355115987e0_f64) * t1285 * t5474 - F::cast_from(0.65854491829355115987e0_f64) * t5478 * t5481 + F::cast_from(0.65854491829355115987e0_f64) * t1204 * t1825 - F::cast_from(0.65854491829355115987e0_f64) * t1234 * t5487 + F::cast_from(0.65854491829355115987e0_f64) * t1285 * t5491 + F::cast_from(0.65854491829355115987e0_f64) * t460 * t5494;
    t5497
}
