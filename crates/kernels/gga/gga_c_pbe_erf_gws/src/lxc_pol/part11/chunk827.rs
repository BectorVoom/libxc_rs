//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 827/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk827<F: Float>(t1314: F, t1215: F, t456: F, t470: F, t174: F, t60: F, t4624: F, t4697: F, t4640: F, t4689: F, t1365: F, t1447: F, t472: F, t1218: F, t542: F, t156: F, t4835: F) -> (F, F, F, F, F, F, F) {
    let t18563 = t1314 * t1314;
    let t18567 = 0.35089340384731224426e1 * t470 * t1215 * t18563 * t456;
    let t18568 = t60 * t174;
    let t18571 = 0.1926377843805564792e1 * t18568 * t4697 * t4624;
    let t18574 = 0.13012297059337829057e0 * t18568 * t4689 * t4640;
    let t18577 = 0.67471169937307261776e-1 * t1447 * t1365 * t472;
    let t18580 = 0.86748647062252193713e-1 * t1447 * t542 * t1218;
    let t18587 = 0.13012297059337829057e0 * t1447 * t156 * t4835;
    (t18563, t18567, t18571, t18574, t18577, t18580, t18587)
}
