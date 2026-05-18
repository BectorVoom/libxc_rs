//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 404/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk404<F: Float>(t119: F, t1306: F, t1309: F, t1608: F, t1611: F, t1615: F, t1909: F, t1915: F, t1938: F, t446: F, t557: F, t850: F, t854: F, t867: F, t882: F) -> F {
    let t1941 = t850 - t854 + F::new(0.13170898365871023197e1) * t1306 - F::new(0.13170898365871023197e1) * t1611 + t867 - F::new(0.13170898365871023197e1) * t1309 + F::new(0.13170898365871023197e1) * t1615 - t882 + F::new(0.65854491829355115987e0) * t119 * t1909 - F::new(0.13170898365871023197e1) * t1608 * t557 + F::new(0.13170898365871023197e1) * t446 * t1915 - F::new(0.65854491829355115987e0) * t446 * t1938;
    t1941
}
