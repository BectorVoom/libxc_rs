//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1562/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1562<F: Float>(t12866: F, t58895: F, t6639: F, t17448: F, t21090: F, t12916: F, t24730: F, t5340: F, t12809: F, t24839: F, t21063: F, t5362: F) -> (F, F, F, F, F) {
    let t83758 = t12866 * t58895 * t6639;
    let t83783 = t17448 * t21090;
    let t83798 = t5340 * t12916 * t24730;
    let t83812 = t12809 * t12916 * t24839;
    let t83849 = t21063 * t5362;
    (t83758, t83783, t83798, t83812, t83849)
}
