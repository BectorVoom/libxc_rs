//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1258/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1258<F: Float>(t106: F, t1147: F, t1182: F, t12532: F, t26905: F, t26949: F, t26997: F, t27045: F, t27095: F, t27143: F, t27194: F, t27248: F, t27255: F, t27259: F, t27266: F, t27277: F, t27278: F, t27286: F, t27345: F, t27419: F, t27492: F, t27549: F, t27614: F, t27684: F, t27753: F, t27824: F, t3164: F, t3170: F, t3171: F, t3264: F, t4410: F, t470: F, t8984: F, t8996: F, t8998: F, t9002: F, t9003: F, t9217: F) -> (F,) {
    let t27831 = 0.27818116767324025134e1 * t106 * (t26905 + t26949 + t26997 + t27045 + t27095 + t27143 + t27194 + t27248) * t470 - 0.11127246706929610054e2 * t106 * t27255 * t1182 + 0.33381740120788830161e2 * t106 * t27259 * t3171 - 0.1669087006039441508e2 * t106 * t8984 * t3264 - 0.66763480241577660323e2 * t106 * t27266 * t8998 + 0.66763480241577660323e2 * t12532 * t9003 - 0.11127246706929610054e2 * t106 * t3164 * t9217 + 0.6676348024157766032e2 * t106 * t27277 * t27278 - 0.10014522036236649048e3 * t4410 * t8996 * t3171 * t3264 + 0.16690870060394415081e2 * t106 * t3170 * t27286 + 0.22254493413859220108e2 * t4410 * t9002 * t9217 - 0.27818116767324025134e1 * t106 * t1147 * (t27345 + t27419 + t27492 + t27549 + t27614 + t27684 + t27753 + t27824);
    (t27831,)
}
