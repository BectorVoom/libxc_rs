//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 784/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk784<F: Float>(t1449: F, t2373: F, t2574: F, t762: F, t2608: F, t6135: F, t10007: F, t14163: F, t24433: F, t1901: F, t24665: F, t24670: F, t24673: F, t24675: F, t24679: F, t24683: F, t24687: F, t24690: F, t24693: F, t24698: F, t24702: F, t24707: F, t24711: F, t24714: F, t446: F) -> (F, F, F, F, F, F) {
    let t24717 = t1449 * t2373;
    let t24719 = t2574 * t762 * t24717;
    let t24722 = t6135 * t2608;
    let t24723 = t10007 * t24722;
    let t24726 = t14163 * t24433;
    let t24729 = 2.0 / 27.0 * t1901 * t24665 - 4.0 / 3.0 * t1901 * t24670 - 2.0 / 27.0 * t24673 + 2.0 / 9.0 * t1901 * t24675 + 2.0 / 3.0 * t446 * t24679 - t446 * t24683 / 9.0 - 2.0 / 27.0 * t446 * t24687 + 2.0 / 27.0 * t24690 - 2.0 / 9.0 * t446 * t24693 + t446 * t24698 / 3.0 + 2.0 / 3.0 * t446 * t24702 + 2.0 / 3.0 * t446 * t24707 + 2.0 / 3.0 * t446 * t24711 - 2.0 * t446 * t24714 - 2.0 / 3.0 * t446 * t24719 - 2.0 / 9.0 * t1901 * t24723 - 4.0 / 9.0 * t1901 * t24726;
    (t24717, t24719, t24722, t24723, t24726, t24729)
}
