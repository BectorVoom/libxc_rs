//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1453/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1453(t1248: f64, t1287: f64, t5412: f64, t1204: f64, t12723: f64, t1281: f64, t1285: f64, t1288: f64, t12987: f64, t17289: f64, t17307: f64, t17861: f64, t17864: f64, t17869: f64, t17876: f64, t17880: f64, t17884: f64, t17888: f64, t17893: f64, t17902: f64, t17905: f64, t1825: f64, t3552: f64, t3666: f64, t3751: f64, t3755: f64, t3782: f64, t5449: f64, t5459: f64, t5466: f64, t5478: f64, t5481: f64, t5494: f64) -> f64 {
    let t17909 = t5412 * t1248 * t1287;
    let t17912 = 0.13170898365871023197e1_f64 * t17861 * t1288 - 0.13170898365871023197e1_f64 * t17864 * t5481 - 0.13170898365871023197e1_f64 * t12723 * t5459 - 0.65854491829355115987e0_f64 * t3782 * t17869 - 0.13170898365871023197e1_f64 * t3666 * t5449 - 0.65854491829355115987e0_f64 * t5478 * t17876 - 0.13170898365871023197e1_f64 * t17880 * t5481 - 0.65854491829355115987e0_f64 * t3755 * t17884 + 0.26341796731742046394e1_f64 * t17888 * t5466 + 0.65854491829355115987e0_f64 * t3552 * t1825 - 0.39512695097613069591e1_f64 * t12987 * t17893 + 0.13170898365871023197e1_f64 * t1204 * t5494 + 0.13170898365871023197e1_f64 * t17307 * t3751 - 0.13170898365871023197e1_f64 * t17289 * t1281 - 0.13170898365871023197e1_f64 * t3755 * t17902 - 0.65854491829355115987e0_f64 * t3755 * t17905 + 0.13170898365871023197e1_f64 * t1285 * t17909;
    t17912
}
