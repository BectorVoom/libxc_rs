//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1322/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1322<F: Float>(t4990: F, t5049: F, t16632: F, t3927: F, t123: F, t1382: F, t4561: F, t4565: F, t16628: F, t10825: F, t11526: F, t1325: F, t14329: F, t14339: F, t14525: F, t14630: F, t14635: F, t16960: F, t16968: F, t16984: F, t17034: F, t17219: F, t25453: F, t25458: F, t2668: F, t2721: F, t2778: F, t2812: F, t297: F, t32252: F, t323: F, t3836: F, t3907: F, t41860: F, t42182: F, t4776: F, t4942: F, t51322: F, t51325: F, t52061: F, t56744: F, t7405: F, t7491: F, t7924: F, t8002: F, t8127: F, t8128: F, t8194: F, t8196: F, t914: F, t930: F) -> (F, F, F, F, F, F) {
    let t57585 = t4990 * t4990;
    let t57592 = t5049 * t5049;
    let t57599 = t16632 * t3927;
    let t57623 = t1382 * t123;
    let t57628 = t4561 * t4565;
    let t57640 = t16628 * t3927;
    let t57657 = -F::cast_from(0.1559479530529405812e3_f64) * t2812 * t3836 * t57599 - F::cast_from(0.11721316454988582616e4_f64) * t41860 + F::cast_from(0.9291736872898228042e2_f64) * t3907 * t14525 * t8002 * t4776 + F::cast_from(0.16829779668897917239e1_f64) * t32252 - F::cast_from(0.61944912485988186948e2_f64) * t7491 * t14329 * t17219 - F::cast_from(0.23967961564076583027e5_f64) * t25453 * t52061 * t17034 + F::cast_from(0.26631068404529536697e4_f64) * t25458 * t52061 * t14339 - F::cast_from(0.13186481011862155443e4_f64) * t2778 * t323 * t56744 * t123 * t297 + F::cast_from(0.35163949364965747848e4_f64) * t11526 * t42182 * t1325 * t57623 + F::cast_from(0.54090782603130048873e0_f64) * t930 * t914 * t7924 * t57628 + F::cast_from(0.71903884692229749079e5_f64) * t8127 * t4942 * t8128 * t16960 - F::cast_from(0.30972456242994093474e2_f64) * t2668 * t14525 * t16968 + F::cast_from(0.47123383072914168269e1_f64) * t2721 * t10825 * t57640 + F::cast_from(0.61944912485988186947e2_f64) * t2668 * t14525 * t16984 + F::cast_from(0.34014423178468276542e6_f64) * t8194 * t14635 * t8196 * t14630 + F::cast_from(0.3118959061058811624e2_f64) * t51322 + F::cast_from(0.3118959061058811624e2_f64) * t51325 - F::cast_from(0.13909058383662012568e1_f64) * t930 * t914 * t7405 * t57628;
    (t57585, t57592, t57599, t57628, t57640, t57657)
}
