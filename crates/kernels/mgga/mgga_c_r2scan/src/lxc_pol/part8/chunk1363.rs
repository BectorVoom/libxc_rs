//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1363/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1363<F: Float>(t113: F, t32444: F, t19905: F, t2155: F, t10012: F, t6118: F, t10041: F, t2139: F, t2294: F, t10073: F, t10046: F, t10010: F, t20511: F, t2124: F, t2557: F, t27661: F, t27775: F, t27914: F, t29469: F, t29473: F, t29475: F, t29478: F, t360: F, t8808: F, t921: F, t9521: F, t9530: F, t9999: F) -> (F, F) {
    let t33352 = t32444 * t113;
    let t33354 = t2155 * t19905 * t33352;
    let t33356 = t6118 * t10012;
    let t33361 = t2139 * t2294 * t10041;
    let t33363 = t6118 * t10073;
    let t33365 = t6118 * t10046;
    let t33377 = -0.31205598264195366828e1 * t27775 * t8808 - 0.20958572791407956061e0 * t29469 - 0.4191714558281591212e0 * t29473 + 0.69861909304693186866e-1 * t29475 - 0.40752780427737692339e0 * t29478 - 0.87816964854445047166e-1 * t33354 - 0.38415120233790484324e0 * t33356 - 0.15602799132097683414e1 * t20511 * t9999 - 0.10401866088065122276e1 * t33361 + 0.76830240467580968648e0 * t33363 - 0.38415120233790484324e0 * t33365 + 0.2600466522016280569e0 * t9521 * t9530 + 0.39006997830244208535e0 * t2139 * t360 * t27914 * t10010 - 0.82318114786693894983e-1 * t2557 * t2124 * t27661 * t921;
    (t33352, t33377)
}
