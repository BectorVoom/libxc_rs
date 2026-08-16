//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 849/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk849<F: Float>(t162: F, t16389: F, t3360: F, t4599: F, t6931: F, t1256: F, t13214: F, t2034: F, t13248: F, t4595: F, t13092: F, t13094: F, t13158: F, t16326: F, t16330: F, t16373: F, t16377: F, t16382: F, t16386: F, t2021: F, t636: F, t6876: F, t6899: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t16390 = t162 * t16389;
    let t16393 = t3360 * t4599;
    let t16394 = t6931 * t16393;
    let t16397 = t13214 * t1256;
    let t16398 = t2034 * t16397;
    let t16401 = t13248 * t1256;
    let t16402 = t2034 * t16401;
    let t16405 = t3360 * t4595;
    let t16406 = t2034 * t16405;
    let t16409 = F::cast_from(0.38032581517825815615e-1_f64) * t13092 + F::cast_from(0.38032581517825815615e-1_f64) * t13094 - F::cast_from(0.7606516303565163123e-1_f64) * t13158 - F::cast_from(0.16299677793353920978e-1_f64) * t6876 * t16326 + F::cast_from(0.16299677793353920977e-1_f64) * t2021 * t16330 - F::cast_from(0.27166129655589868296e-2_f64) * t636 * t16373 - F::cast_from(0.27166129655589868296e-2_f64) * t636 * t16377 - t6899 - F::cast_from(0.81498388966769604888e-2_f64) * t636 * t16382 - F::cast_from(0.65198711173415683909e-1_f64) * t2021 * t16386 + F::cast_from(0.16299677793353920977e-1_f64) * t2021 * t16390 - F::cast_from(0.16299677793353920977e0_f64) * t636 * t16394 + F::cast_from(0.32599355586707841954e-1_f64) * t636 * t16398 + F::cast_from(0.32599355586707841954e-1_f64) * t636 * t16402 + F::cast_from(0.32599355586707841954e-1_f64) * t636 * t16406;
    (t16390, t16393, t16394, t16397, t16398, t16401, t16402, t16405, t16406, t16409)
}
