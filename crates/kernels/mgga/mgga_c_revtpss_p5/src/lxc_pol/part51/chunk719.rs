//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 719/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk719<F: Float>(t1916: F, t2042: F, t1518: F, t7330: F, t572: F, t117: F, t7741: F, t1918: F, t2040: F, t573: F, t7944: F, t38: F) -> (F, F, F, F) {
    let t7949 = F::cast_from(3.0_f64) * t1916 * t2042;
    let t7950 = t7330 * t1518;
    let t7952 = F::cast_from(6.0_f64) * t572 * t7950;
    let t7953 = t117 * t7741;
    let t7955 = F::cast_from(3.0_f64) * t572 * t7953;
    let t7956 = F::cast_from(3.0_f64) * t1918 * t2040 + t573 * t7944 + t7949 + t7952 + t7955;
    let t8435 = t38 * t38;
    (t7950, t7953, t7956, t8435)
}
