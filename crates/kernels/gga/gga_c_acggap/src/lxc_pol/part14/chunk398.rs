//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 398/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk398<F: Float>(t119: F, t1306: F, t1309: F, t1608: F, t1611: F, t1615: F, t1909: F, t1915: F, t1938: F, t446: F, t557: F, t850: F, t854: F, t867: F, t882: F, t104: F, t624: F) -> (F, F) {
    let t1941 = t850 - t854 + 0.13170898365871023197e1 * t1306 - 0.13170898365871023197e1 * t1611 + t867 - 0.13170898365871023197e1 * t1309 + 0.13170898365871023197e1 * t1615 - t882 + 0.65854491829355115987e0 * t119 * t1909 - 0.13170898365871023197e1 * t1608 * t557 + 0.13170898365871023197e1 * t446 * t1915 - 0.65854491829355115987e0 * t446 * t1938;
    let t1953 = t104 * t624;
    (t1941, t1953)
}
