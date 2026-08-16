//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1465/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1465(t16543: f64, t4746: f64, t3057: f64, t6343: f64, t15669: f64, t1678: f64, t2435: f64, t6430: f64) -> (f64, f64, f64, f64) {
    let t67927 = t4746 * t16543;
    let t68022 = t3057 * t6343;
    let t68144 = t15669 * t1678;
    let t68255 = t2435 * t6430;
    (t67927, t68022, t68144, t68255)
}
