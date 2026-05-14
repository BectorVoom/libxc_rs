//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 825/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk825<F: Float>(t18486: F, t18488: F, t18491: F, t18494: F, t18500: F, t18502: F, t18504: F, t18506: F, t389: F, t404: F, t7236: F, t7271: F, t1275: F, t1293: F, t4659: F, t1285: F, t4661: F) -> (F, F, F, F) {
    let t18512 = 1.0 * t389 * (-0.21099166666666666667e1 * t18486 + 0.202552e2 * t18488 - 0.75019259259259259258e1 * t18491 + 0.6564185185185185185e1 * t18494 + 0.31003950617283950618e1 * t7271 + 0.68258333333333333335e-1 * t18500 - 0.10921333333333333333e1 * t18502 + 0.12134814814814814815e1 * t18504 + 0.10617962962962962963e1 * t18506 + 0.13388493827160493828e1 * t7236) * t404;
    let t18515 = t1275 * t1275;
    let t18518 = 0.57894567559743977359e3 * t4659 * t18515 * t1293;
    let t18527 = 0.3103500882342370105e4 * t4659 * t1275 * t4661 * t1285;
    (t18512, t18515, t18518, t18527)
}
