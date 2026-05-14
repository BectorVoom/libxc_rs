//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1268/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1268<F: Float>(t3151: F, t55922: F, t894: F, t27221: F, t55917: F, t27100: F, t1111: F, t1133: F, t11596: F, t15327: F, t17700: F, t17705: F, t17710: F, t17893: F, t17904: F, t322: F, t3245: F, t34387: F, t34390: F, t34393: F, t35441: F, t35453: F, t4289: F, t4310: F, t46820: F, t46832: F, t5297: F, t5298: F, t54341: F, t58346: F, t58350: F, t58354: F, t58365: F, t58923: F, t58928: F, t8966: F, t8973: F) -> (F, F, F, F) {
    let t59503 = t894 * t3151 * t55922;
    let t59511 = t894 * t27221 * t55917;
    let t59527 = t27100 * t55917;
    let t59531 = 0.19535527424980971027e3 * t35441 * t17893 - 0.17171677016866682182e-1 * t35453 + 0.36704459623552533164e0 * t15327 * t5298 - t1111 * t3245 * t58346 / 36.0 - t1111 * t4289 * t58350 / 6.0 + t1111 * t4289 * t58365 / 54.0 + 7.0 / 108.0 * t1111 * t11596 * t58354 - 0.10866451862235947318e-1 * t1133 * t59503 - 0.22676282118978851027e6 * t34390 * t17705 + 0.22676282118978851028e6 * t34393 * t17710 - 0.43465807448943789272e-1 * t1133 * t59511 - 0.3779380353163141838e5 * t34387 * t17700 + 0.50489339006693751717e0 * t46820 - 0.24147670804968771818e-2 * t46832 + 0.48838818562452427568e2 * t54341 + 0.36629113921839320676e2 * t8973 * t5297 * t58923 - 0.18314556960919660338e2 * t8966 * t5297 * t58928 - t4310 * t17904 / 27.0 + t1111 * t322 * t59527 / 6.0;
    (t59503, t59511, t59527, t59531)
}
