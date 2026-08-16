//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1425/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1425(t3151: f64, t55922: f64, t894: f64, t27221: f64, t55917: f64, t27100: f64, t1111: f64, t1133: f64, t11596: f64, t15327: f64, t17700: f64, t17705: f64, t17710: f64, t17893: f64, t17904: f64, t322: f64, t3245: f64, t34387: f64, t34390: f64, t34393: f64, t35441: f64, t35453: f64, t4289: f64, t4310: f64, t46820: f64, t46832: f64, t5297: f64, t5298: f64, t54341: f64, t58346: f64, t58350: f64, t58354: f64, t58365: f64, t58923: f64, t58928: f64, t8966: f64, t8973: f64) -> (f64, f64, f64, f64) {
    let t59503 = t894 * t3151 * t55922;
    let t59511 = t894 * t27221 * t55917;
    let t59527 = t27100 * t55917;
    let t59531 = 0.19535527424980971027e3_f64 * t35441 * t17893 - 0.17171677016866682182e-1_f64 * t35453 + 0.36704459623552533164e0_f64 * t15327 * t5298 - t1111 * t3245 * t58346 / 36.0_f64 - t1111 * t4289 * t58350 / 6.0_f64 + t1111 * t4289 * t58365 / 54.0_f64 + 7.0_f64 / 108.0_f64 * t1111 * t11596 * t58354 - 0.10866451862235947318e-1_f64 * t1133 * t59503 - 0.22676282118978851027e6_f64 * t34390 * t17705 + 0.22676282118978851028e6_f64 * t34393 * t17710 - 0.43465807448943789272e-1_f64 * t1133 * t59511 - 0.3779380353163141838e5_f64 * t34387 * t17700 + 0.50489339006693751717e0_f64 * t46820 - 0.24147670804968771818e-2_f64 * t46832 + 0.48838818562452427568e2_f64 * t54341 + 0.36629113921839320676e2_f64 * t8973 * t5297 * t58923 - 0.18314556960919660338e2_f64 * t8966 * t5297 * t58928 - t4310 * t17904 / 27.0_f64 + t1111 * t322 * t59527 / 6.0_f64;
    (t59503, t59511, t59527, t59531)
}
