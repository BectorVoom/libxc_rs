//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1062/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1062<F: Float>(t32677: F, t8707: F, t32287: F, t32266: F, t32270: F, t1444: F, t8708: F, t32250: F, t1032: F, t2097: F, t1955: F) -> (F, F, F, F, F, F, F, F) {
    let t32678 = t8707 * t32677;
    let t32681 = F::new(0.17354086964223805049e-2) * t32287;
    let t32682 = F::new(0.3718732920905101082e-4) * t32266;
    let t32683 = F::new(0.66119071333692697238e-4) * t32270;
    let t32685 = t8708 * t1444;
    let t32686 = t32250 * t32685;
    let t32689 = t2097 * t1032;
    let t32690 = t1955 * t32689;
    (t32678, t32681, t32682, t32683, t32685, t32686, t32689, t32690)
}
