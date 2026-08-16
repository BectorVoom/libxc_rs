//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2980/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2980(t11247: f64, t1651: f64, t16087: f64, t53884: f64, t15988: f64, t3241: f64, t1011: f64, t15158: f64, t15987: f64, t11250: f64, t11257: f64, t11632: f64, t15997: f64, t16000: f64, t16091: f64, t3117: f64, t42417: f64, t42621: f64, t42690: f64, t42781: f64, t42785: f64, t43105: f64, t4788: f64, t4915: f64, t52002: f64) -> (f64, f64) {
    let t54276 = t1651 * t11247;
    let t54289 = t16087 * t53884;
    let t54303 = t3241 * t15988;
    let t54306 = t1011 * t15987 * t15158;
    let t54308 = -0.12862205435420921092e-2_f64 * t42621 * t3117 * t54276 * t11632 + 0.12862205435420921092e-2_f64 * t43105 * t3117 * t54276 * t11250 - 0.21437009059034868486e-3_f64 * t42690 * t3117 * t54276 * t11257 - 0.91464571985215438873e-2_f64 * t54289 * t16091 + 0.1270341277572436651e-3_f64 * t42781 + 0.19055119163586549765e-3_f64 * t42785 + 0.14481890564325777822e-1_f64 * t42417 * t4788 + t3241 * t15997 / 9.0_f64 + t1011 * t4915 * t52002 / 16.0_f64 + t3241 * t16000 / 18.0_f64 + t54303 / 27.0_f64 + t54306 / 48.0_f64;
    (t54276, t54308)
}
