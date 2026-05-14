//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 838/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk838<F: Float>(t2286: F, t38355: F, t7720: F, t9935: F, t10106: F, t16043: F, t10088: F, t2144: F, t3351: F, t352: F, t7231: F, t1704: F, t2084: F, t27: F, t7282: F, t39667: F, t39679: F, t39694: F, t39698: F, t39702: F, t45757: F, t45759: F, t45763: F, t45767: F, t45769: F, t45775: F, t4985: F, t739: F, t8960: F) -> (F,) {
    let t45777 = t38355 * t2286;
    let t45779 = t7720 * t9935;
    let t45781 = t16043 * t10106;
    let t45788 = t3351 * t7231 * t2144 * t10088 * t352;
    let t45794 = t7282 * t27 * t2084 * t1704;
    let t45796 = -0.25538759935978703639e-4 * t45757 - 0.85129199786595678796e-5 * t45759 + 0.85129199786595678796e-5 * t45763 - 0.53205749866622299248e-5 * t45767 - 0.59871208509319042821e-1 * t739 * t45769 + 0.54549323308490683458e-1 * t39667 - 0.27274661654245341728e-1 * t45775 + 0.25538759935978703638e-4 * t45777 + 0.25538759935978703638e-4 * t45779 + t39679 - 0.76616279807936110914e-4 * t45781 + 0.21819729323396273383e0 * t39694 + 0.54549323308490683457e-1 * t39698 - t39702 - 0.25538759935978703638e-4 * t45788 + 0.11974241701863808564e0 * t4985 * t8960 - 0.54549323308490683456e-1 * t45794;
    (t45796,)
}
