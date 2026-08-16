//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1396/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1396(t106: f64, t1147: f64, t1182: f64, t12532: f64, t26905: f64, t26949: f64, t26997: f64, t27045: f64, t27095: f64, t27143: f64, t27194: f64, t27248: f64, t27255: f64, t27259: f64, t27266: f64, t27277: f64, t27278: f64, t27286: f64, t27345: f64, t27419: f64, t27492: f64, t27549: f64, t27614: f64, t27684: f64, t27753: f64, t27824: f64, t3164: f64, t3170: f64, t3171: f64, t3264: f64, t4410: f64, t470: f64, t8984: f64, t8996: f64, t8998: f64, t9002: f64, t9003: f64, t9217: f64) -> f64 {
    let t27831 = 0.27818116767324025134e1_f64 * t106 * (t26905 + t26949 + t26997 + t27045 + t27095 + t27143 + t27194 + t27248) * t470 - 0.11127246706929610054e2_f64 * t106 * t27255 * t1182 + 0.33381740120788830161e2_f64 * t106 * t27259 * t3171 - 0.1669087006039441508e2_f64 * t106 * t8984 * t3264 - 0.66763480241577660323e2_f64 * t106 * t27266 * t8998 + 0.66763480241577660323e2_f64 * t12532 * t9003 - 0.11127246706929610054e2_f64 * t106 * t3164 * t9217 + 0.6676348024157766032e2_f64 * t106 * t27277 * t27278 - 0.10014522036236649048e3_f64 * t4410 * t8996 * t3171 * t3264 + 0.16690870060394415081e2_f64 * t106 * t3170 * t27286 + 0.22254493413859220108e2_f64 * t4410 * t9002 * t9217 - 0.27818116767324025134e1_f64 * t106 * t1147 * (t27345 + t27419 + t27492 + t27549 + t27614 + t27684 + t27753 + t27824);
    t27831
}
