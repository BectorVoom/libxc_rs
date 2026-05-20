//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1584/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1584<F: Float>(t10022: F, t2782: F, t86413: F, t1882: F, t6888: F, t22857: F, t555: F, t22953: F, t22954: F, t4101: F, t686: F, t72: F) -> (F, F, F, F, F) {
    let t86415 = t2782 * t10022 * t86413;
    let t86441 = t6888 * t1882;
    let t86445 = t555 * t22857;
    let t86455 = t555 * t22953;
    let t86468 = t4101 * t22954 * t72 * t686;
    (t86415, t86441, t86445, t86455, t86468)
}
