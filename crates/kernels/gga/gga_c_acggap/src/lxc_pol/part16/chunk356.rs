//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 356/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk356<F: Float>(t119: F, t1215: F, t1306: F, t1309: F, t1605: F, t1608: F, t1611: F, t1615: F, t1620: F, t1659: F, t446: F, t464: F, t557: F, t850: F, t854: F, t855: F, t858: F, t867: F, t869: F, t873: F, t882: F) -> F {
    let t1662 = t850 - t854 + F::new(0.65854491829355115987e0) * t855 - F::new(0.65854491829355115987e0) * t858 + t867 - F::new(0.65854491829355115987e0) * t869 + F::new(0.65854491829355115987e0) * t873 - t882 + F::new(0.65854491829355115987e0) * t1306 - F::new(0.65854491829355115987e0) * t1309 + F::new(0.65854491829355115987e0) * t119 * t1605 - F::new(0.65854491829355115987e0) * t1608 * t464 - F::new(0.65854491829355115987e0) * t1611 + F::new(0.65854491829355115987e0) * t1615 - F::new(0.65854491829355115987e0) * t1215 * t557 + F::new(0.13170898365871023197e1) * t446 * t1620 - F::new(0.65854491829355115987e0) * t446 * t1659;
    t1662
}
