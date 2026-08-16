//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1274/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1274<F: Float>(t5795: F, t8731: F, t2113: F, t28268: F, t7334: F, t8118: F, t28280: F, t1916: F, t32779: F, t60221: F, t8736: F, t13272: F, t32805: F) -> (F, F, F, F, F, F, F) {
    let t129099 = F::cast_from(6.0_f64) * t5795 * t8731;
    let t129103 = F::cast_from(6.0_f64) * t2113 * t28268;
    let t129107 = F::cast_from(3.0_f64) * t8118 * t7334;
    let t129109 = F::cast_from(3.0_f64) * t2113 * t28280;
    let t129111 = F::cast_from(6.0_f64) * t1916 * t32779;
    let t129157 = t60221 * t8736;
    let t129160 = t13272 * t32805;
    (t129099, t129103, t129107, t129109, t129111, t129157, t129160)
}
