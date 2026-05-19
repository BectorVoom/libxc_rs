//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1046/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1046<F: Float>(t11334: F, t11337: F, t11339: F, t11345: F, t11348: F, t11351: F, t11353: F, t11358: F, t11369: F, t11304: F, t11306: F, t11309: F, t11330: F, t11363: F, t11367: F, t12068: F, t12069: F, t12070: F, t12071: F) -> F {
    let t12073 = F::cast_from(0.25340269868817520617e-3_f64) * t11334;
    let t12074 = F::cast_from(0.25301920572916666668e-5_f64) * t11337;
    let t12075 = F::cast_from(0.40483072916666666669e-4_f64) * t11339;
    let t12076 = F::cast_from(0.24458523220486111112e-4_f64) * t11345;
    let t12077 = F::cast_from(0.34752370105806885418e-3_f64) * t11348;
    let t12078 = F::cast_from(0.40483072916666666669e-4_f64) * t11351;
    let t12079 = F::cast_from(0.10821235962619981449e-3_f64) * t11353;
    let t12080 = F::cast_from(0.42206481990611010728e-7_f64) * t11358;
    let t12083 = F::cast_from(0.13259557375557346398e-6_f64) * t11369;
    let t12084 = F::cast_from(0.18115908419564701085e-6_f64) * t11304 - F::cast_from(0.18115908419564701085e-6_f64) * t11306 + F::cast_from(0.66297786877786731988e-7_f64) * t11309 + t12068 + t12069 - t12070 - t12071 + F::cast_from(0.66297786877786731988e-7_f64) * t11330 - t12073 - t12074 - t12075 + t12076 - t12077 - t12078 + t12079 - t12080 + F::cast_from(0.78584976712469872986e-8_f64) * t11363 - F::cast_from(0.52838066223730378165e-7_f64) * t11367 - t12083;
    t12084
}
