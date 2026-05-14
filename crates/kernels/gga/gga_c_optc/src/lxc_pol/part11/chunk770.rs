//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 770/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk770<F: Float>(t4215: F, t5250: F, t5257: F, t1107: F, t190: F, t5245: F, t5243: F, t11782: F, t5228: F, t4297: F, t5087: F, t9254: F, t2911: F, t5434: F, t1013: F, t4298: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15099 = t5250 * t4215;
    let t15101 = t5257 * t4215;
    let t15104 = t1107 * t190 * t5245;
    let t15105 = t5243 * t15104;
    let t15107 = t11782 * t5228;
    let t15108 = t4297 * t15107;
    let t15122 = t5087 * t9254;
    let t15138 = t5434 * t2911;
    let t15142 = t4298 * t1013;
    (t15099, t15101, t15104, t15105, t15107, t15108, t15122, t15138, t15142)
}
