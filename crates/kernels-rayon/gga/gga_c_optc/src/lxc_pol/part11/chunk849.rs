//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 849/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk849(t162: f64, t16389: f64, t3360: f64, t4599: f64, t6931: f64, t1256: f64, t13214: f64, t2034: f64, t13248: f64, t4595: f64, t13092: f64, t13094: f64, t13158: f64, t16326: f64, t16330: f64, t16373: f64, t16377: f64, t16382: f64, t16386: f64, t2021: f64, t636: f64, t6876: f64, t6899: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16390 = t162 * t16389;
    let t16393 = t3360 * t4599;
    let t16394 = t6931 * t16393;
    let t16397 = t13214 * t1256;
    let t16398 = t2034 * t16397;
    let t16401 = t13248 * t1256;
    let t16402 = t2034 * t16401;
    let t16405 = t3360 * t4595;
    let t16406 = t2034 * t16405;
    let t16409 = 0.38032581517825815615e-1_f64 * t13092 + 0.38032581517825815615e-1_f64 * t13094 - 0.7606516303565163123e-1_f64 * t13158 - 0.16299677793353920978e-1_f64 * t6876 * t16326 + 0.16299677793353920977e-1_f64 * t2021 * t16330 - 0.27166129655589868296e-2_f64 * t636 * t16373 - 0.27166129655589868296e-2_f64 * t636 * t16377 - t6899 - 0.81498388966769604888e-2_f64 * t636 * t16382 - 0.65198711173415683909e-1_f64 * t2021 * t16386 + 0.16299677793353920977e-1_f64 * t2021 * t16390 - 0.16299677793353920977e0_f64 * t636 * t16394 + 0.32599355586707841954e-1_f64 * t636 * t16398 + 0.32599355586707841954e-1_f64 * t636 * t16402 + 0.32599355586707841954e-1_f64 * t636 * t16406;
    (t16390, t16393, t16394, t16397, t16398, t16401, t16402, t16405, t16406, t16409)
}
