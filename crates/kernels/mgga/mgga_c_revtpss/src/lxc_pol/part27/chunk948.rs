//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 948/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk948<F: Float>(t12810: F, t3629: F, t3626: F, t221: F, t462: F, t68: F, t461: F, t1209: F, t3766: F, t5330: F, t1214: F, t3603: F, t3720: F, t1250: F, t12726: F, t11772: F, t3623: F) -> (F, F, F, F, F, F, F, F) {
    let t12846 = t12810 * t3629;
    let t12847 = t3626 * t12846;
    let t12851 = t221 * t68 * t462;
    let t12853 = 5.0 / 1296.0 * t461 * t12851;
    let t12854 = t1209 * t3766;
    let t12855 = t12854 * t5330;
    let t12856 = t3603 * t1214;
    let t12857 = t12810 * t12856;
    let t12858 = t3720 * t12857;
    let t12861 = t12726 * t1250;
    let t12862 = t3720 * t12861;
    let t12865 = t3623 * t11772;
    (t12847, t12851, t12853, t12854, t12855, t12858, t12862, t12865)
}
