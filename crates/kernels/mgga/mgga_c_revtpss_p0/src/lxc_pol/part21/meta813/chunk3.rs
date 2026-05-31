//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2980/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2980<F: Float>(t11247: F, t1651: F, t16087: F, t53884: F, t15988: F, t3241: F, t1011: F, t15158: F, t15987: F, t11250: F, t11257: F, t11632: F, t15997: F, t16000: F, t16091: F, t3117: F, t42417: F, t42621: F, t42690: F, t42781: F, t42785: F, t43105: F, t4788: F, t4915: F, t52002: F) -> (F, F) {
    let t54276 = t1651 * t11247;
    let t54289 = t16087 * t53884;
    let t54303 = t3241 * t15988;
    let t54306 = t1011 * t15987 * t15158;
    let t54308 = -F::cast_from(0.12862205435420921092e-2_f64) * t42621 * t3117 * t54276 * t11632 + F::cast_from(0.12862205435420921092e-2_f64) * t43105 * t3117 * t54276 * t11250 - F::cast_from(0.21437009059034868486e-3_f64) * t42690 * t3117 * t54276 * t11257 - F::cast_from(0.91464571985215438873e-2_f64) * t54289 * t16091 + F::cast_from(0.1270341277572436651e-3_f64) * t42781 + F::cast_from(0.19055119163586549765e-3_f64) * t42785 + F::cast_from(0.14481890564325777822e-1_f64) * t42417 * t4788 + t3241 * t15997 / F::cast_from(9.0_f64) + t1011 * t4915 * t52002 / F::cast_from(16.0_f64) + t3241 * t16000 / F::cast_from(18.0_f64) + t54303 / F::cast_from(27.0_f64) + t54306 / F::cast_from(48.0_f64);
    (t54276, t54308)
}
