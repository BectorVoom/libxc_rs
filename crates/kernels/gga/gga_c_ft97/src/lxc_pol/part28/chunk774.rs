//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 774/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk774<F: Float>(t34660: F, t34693: F, t34735: F, t34782: F, t1564: F, t32423: F, t925: F, t1286: F, t1310: F, t32016: F, t34553: F, t34557: F, t34560: F, t34563: F, t34566: F, t34569: F, t34577: F, t34581: F, t34585: F, t34589: F, t34614: F, t34620: F, t5501: F, t6414: F, t6418: F, t6457: F, t7162: F, t7214: F, t7218: F, t7286: F, t88: F, t948: F) -> (F, F, F) {
    let t34784 = t34660 + t34693 + t34735 + t34782;
    let t34787 = t1564 * t32423 * t925;
    let t34790 = -t948 * t7286 - t32016 * t6418 / 18.0 - t5501 * t34553 / 18.0 + t5501 * t34557 / 9.0 + 8.0 * t34560 - 12.0 * t34563 + 8.0 * t34566 + 4.0 * t34569 + t6414 * t7214 / 6.0 + t6414 * t7218 / 3.0 + t1286 * t34577 / 6.0 + t1286 * t34581 / 6.0 + t1286 * t34585 / 3.0 + t1286 * t34589 / 3.0 + t34614 * t1310 / 6.0 + t7162 * t6457 / 6.0 - t1286 * t34620 / 3.0 - t88 * t34784 - t5501 * t34787 / 9.0;
    (t34784, t34787, t34790)
}
