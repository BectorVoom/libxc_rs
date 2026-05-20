//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2405/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2405<F: Float>(t10988: F, t2435: F, t2445: F, t9292: F, t11025: F, t10981: F, t588: F, t780: F, t10991: F, t39497: F, t787: F, t788: F) -> (F, F, F, F, F, F) {
    let t40986 = t2435 * t10988;
    let t40988 = t9292 * t2445;
    let t40994 = t2435 * t11025;
    let t40998 = F::cast_from(0.15709759505761725819e-2_f64) * t10981 * t780 * t588;
    let t40999 = t2435 * t10991;
    let t41003 = F::cast_from(0.10118827226026589797e0_f64) * t787 * t788 * t39497;
    (t40986, t40988, t40994, t40998, t40999, t41003)
}
