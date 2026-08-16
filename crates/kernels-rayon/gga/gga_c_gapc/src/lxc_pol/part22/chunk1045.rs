//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1045/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1045(t11334: f64, t11337: f64, t11339: f64, t11345: f64, t11348: f64, t11351: f64, t11353: f64, t11358: f64, t11369: f64, t11304: f64, t11306: f64, t11309: f64, t11330: f64, t11363: f64, t11367: f64, t12068: f64, t12069: f64, t12070: f64, t12071: f64) -> f64 {
    let t12073 = 0.25340269868817520617e-3_f64 * t11334;
    let t12074 = 0.25301920572916666668e-5_f64 * t11337;
    let t12075 = 0.40483072916666666669e-4_f64 * t11339;
    let t12076 = 0.24458523220486111112e-4_f64 * t11345;
    let t12077 = 0.34752370105806885418e-3_f64 * t11348;
    let t12078 = 0.40483072916666666669e-4_f64 * t11351;
    let t12079 = 0.10821235962619981449e-3_f64 * t11353;
    let t12080 = 0.42206481990611010728e-7_f64 * t11358;
    let t12083 = 0.13259557375557346398e-6_f64 * t11369;
    let t12084 = 0.18115908419564701085e-6_f64 * t11304 - 0.18115908419564701085e-6_f64 * t11306 + 0.66297786877786731988e-7_f64 * t11309 + t12068 + t12069 - t12070 - t12071 + 0.66297786877786731988e-7_f64 * t11330 - t12073 - t12074 - t12075 + t12076 - t12077 - t12078 + t12079 - t12080 + 0.78584976712469872986e-8_f64 * t11363 - 0.52838066223730378165e-7_f64 * t11367 - t12083;
    t12084
}
