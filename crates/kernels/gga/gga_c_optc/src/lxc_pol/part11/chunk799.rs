//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 799/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk799<F: Float>(t16397: F, t2034: F, t1256: F, t13248: F, t3360: F, t4595: F, t13092: F, t13094: F, t13158: F, t16326: F, t16330: F, t16373: F, t16377: F, t16382: F, t16386: F, t16390: F, t16394: F, t2021: F, t636: F, t6876: F, t6899: F) -> (F, F, F, F, F, F) {
    let t16398 = t2034 * t16397;
    let t16401 = t13248 * t1256;
    let t16402 = t2034 * t16401;
    let t16405 = t3360 * t4595;
    let t16406 = t2034 * t16405;
    let t16409 = 0.38032581517825815615e-1 * t13092 + 0.38032581517825815615e-1 * t13094 - 0.7606516303565163123e-1 * t13158 - 0.16299677793353920978e-1 * t6876 * t16326 + 0.16299677793353920977e-1 * t2021 * t16330 - 0.27166129655589868296e-2 * t636 * t16373 - 0.27166129655589868296e-2 * t636 * t16377 - t6899 - 0.81498388966769604888e-2 * t636 * t16382 - 0.65198711173415683909e-1 * t2021 * t16386 + 0.16299677793353920977e-1 * t2021 * t16390 - 0.16299677793353920977e0 * t636 * t16394 + 0.32599355586707841954e-1 * t636 * t16398 + 0.32599355586707841954e-1 * t636 * t16402 + 0.32599355586707841954e-1 * t636 * t16406;
    (t16398, t16401, t16402, t16405, t16406, t16409)
}
