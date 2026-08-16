//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1733/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1733<F: Float>(t1248: F, t1287: F, t5412: F, t1204: F, t12723: F, t1281: F, t1285: F, t1288: F, t12987: F, t17289: F, t17307: F, t17861: F, t17864: F, t17869: F, t17876: F, t17880: F, t17884: F, t17888: F, t17893: F, t17902: F, t17905: F, t1825: F, t3552: F, t3666: F, t3751: F, t3755: F, t3782: F, t5449: F, t5459: F, t5466: F, t5478: F, t5481: F, t5494: F) -> F {
    let t17909 = t5412 * t1248 * t1287;
    let t17912 = F::cast_from(0.13170898365871023197e1_f64) * t17861 * t1288 - F::cast_from(0.13170898365871023197e1_f64) * t17864 * t5481 - F::cast_from(0.13170898365871023197e1_f64) * t12723 * t5459 - F::cast_from(0.65854491829355115987e0_f64) * t3782 * t17869 - F::cast_from(0.13170898365871023197e1_f64) * t3666 * t5449 - F::cast_from(0.65854491829355115987e0_f64) * t5478 * t17876 - F::cast_from(0.13170898365871023197e1_f64) * t17880 * t5481 - F::cast_from(0.65854491829355115987e0_f64) * t3755 * t17884 + F::cast_from(0.26341796731742046394e1_f64) * t17888 * t5466 + F::cast_from(0.65854491829355115987e0_f64) * t3552 * t1825 - F::cast_from(0.39512695097613069591e1_f64) * t12987 * t17893 + F::cast_from(0.13170898365871023197e1_f64) * t1204 * t5494 + F::cast_from(0.13170898365871023197e1_f64) * t17307 * t3751 - F::cast_from(0.13170898365871023197e1_f64) * t17289 * t1281 - F::cast_from(0.13170898365871023197e1_f64) * t3755 * t17902 - F::cast_from(0.65854491829355115987e0_f64) * t3755 * t17905 + F::cast_from(0.13170898365871023197e1_f64) * t1285 * t17909;
    t17912
}
