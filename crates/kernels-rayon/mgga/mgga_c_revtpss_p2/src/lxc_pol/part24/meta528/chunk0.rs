//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1562/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1562(t12866: f64, t58895: f64, t6639: f64, t17448: f64, t21090: f64, t12916: f64, t24730: f64, t5340: f64, t12809: f64, t24839: f64, t21063: f64, t5362: f64) -> (f64, f64, f64, f64, f64) {
    let t83758 = t12866 * t58895 * t6639;
    let t83783 = t17448 * t21090;
    let t83798 = t5340 * t12916 * t24730;
    let t83812 = t12809 * t12916 * t24839;
    let t83849 = t21063 * t5362;
    (t83758, t83783, t83798, t83812, t83849)
}
