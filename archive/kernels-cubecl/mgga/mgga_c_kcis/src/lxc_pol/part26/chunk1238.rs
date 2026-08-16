//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1238/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1238<F: Float>(t17449: F, t491: F, t11825: F, t27543: F, t1928: F, t4248: F, t5747: F, t5998: F, t1528: F, t4254: F, t572: F, t2060: F, sigma2: F) -> (F, F, F, F, F, F, F, F) {
    let t97701 = t17449 * t491;
    let t97706 = t11825 * t27543;
    let t97727 = t4248 * t1928;
    let t97767 = t5747 * t27543;
    let t97772 = t5998 * t491;
    let t97784 = t1528 * t1928;
    let t97793 = t4254 * t572;
    let t97800 = sigma2 * t2060;
    (t97701, t97706, t97727, t97767, t97772, t97784, t97793, t97800)
}
