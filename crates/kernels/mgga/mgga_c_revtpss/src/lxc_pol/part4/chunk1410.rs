//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1410/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1410<F: Float>(t1248: F, t1287: F, t5412: F, t1204: F, t12723: F, t1281: F, t1285: F, t1288: F, t12987: F, t17289: F, t17307: F, t17861: F, t17864: F, t17869: F, t17876: F, t17880: F, t17884: F, t17888: F, t17893: F, t17902: F, t17905: F, t1825: F, t3552: F, t3666: F, t3751: F, t3755: F, t3782: F, t5449: F, t5459: F, t5466: F, t5478: F, t5481: F, t5494: F) -> F {
    let t17909 = t5412 * t1248 * t1287;
    let t17912 = F::new(0.13170898365871023197e1) * t17861 * t1288 - F::new(0.13170898365871023197e1) * t17864 * t5481 - F::new(0.13170898365871023197e1) * t12723 * t5459 - F::new(0.65854491829355115987e0) * t3782 * t17869 - F::new(0.13170898365871023197e1) * t3666 * t5449 - F::new(0.65854491829355115987e0) * t5478 * t17876 - F::new(0.13170898365871023197e1) * t17880 * t5481 - F::new(0.65854491829355115987e0) * t3755 * t17884 + F::new(0.26341796731742046394e1) * t17888 * t5466 + F::new(0.65854491829355115987e0) * t3552 * t1825 - F::new(0.39512695097613069591e1) * t12987 * t17893 + F::new(0.13170898365871023197e1) * t1204 * t5494 + F::new(0.13170898365871023197e1) * t17307 * t3751 - F::new(0.13170898365871023197e1) * t17289 * t1281 - F::new(0.13170898365871023197e1) * t3755 * t17902 - F::new(0.65854491829355115987e0) * t3755 * t17905 + F::new(0.13170898365871023197e1) * t1285 * t17909;
    t17912
}
