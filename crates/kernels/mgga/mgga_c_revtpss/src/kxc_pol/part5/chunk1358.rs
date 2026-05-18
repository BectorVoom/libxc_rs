//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1358/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1358<F: Float>(t1287: F, t1794: F, t5412: F, t5245: F, t5486: F, t1204: F, t1234: F, t12717: F, t1281: F, t1285: F, t17192: F, t17289: F, t17846: F, t17853: F, t1818: F, t20850: F, t21579: F, t21583: F, t21587: F, t21592: F, t21596: F, t21599: F, t3666: F, t3670: F, t3746: F, t5326: F, t5436: F, t5449: F, t5452: F, t5459: F, t5463: F, t5474: F, t5481: F, t6723: F, t6735: F, t6741: F) -> F {
    let t21607 = t5412 * t1794 * t1287;
    let t21610 = t5486 * t5245;
    let t21615 = -F::new(0.13170898365871023197e1) * t5326 * t5449 - F::new(0.13170898365871023197e1) * t17289 * t1818 + F::new(0.13170898365871023197e1) * t5436 * t5474 - F::new(0.13170898365871023197e1) * t5326 * t5452 + F::new(0.65854491829355115987e0) * t1204 * t6741 - F::new(0.13170898365871023197e1) * t21579 * t5481 + F::new(0.39512695097613069591e1) * t17846 * t21583 - F::new(0.39512695097613069591e1) * t17853 * t21587 - F::new(0.13170898365871023197e1) * t17192 * t5459 + F::new(0.26341796731742046394e1) * t3670 * t21592 + F::new(0.26341796731742046394e1) * t5463 * t21596 + F::new(0.13170898365871023197e1) * t12717 * t21599 - F::new(0.65854491829355115987e0) * t20850 * t1281 + F::new(0.65854491829355115987e0) * t3746 * t6735 + F::new(0.13170898365871023197e1) * t1285 * t21607 - F::new(0.13170898365871023197e1) * t1234 * t21610 - F::new(0.65854491829355115987e0) * t3666 * t6723;
    t21615
}
