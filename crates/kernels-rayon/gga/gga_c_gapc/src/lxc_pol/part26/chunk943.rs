//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 943/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk943(t11522: f64, t5541: f64, t8677: f64, t5462: f64, t8681: f64, t3670: f64, t620: f64, t11466: f64, t11469: f64, t11471: f64, t11475: f64, t11477: f64, t11481: f64, t11486: f64, t11490: f64, t11493: f64, t11497: f64, t11501: f64, t11504: f64, t11506: f64, t11510: f64, t11515: f64, t11520: f64) -> (f64, f64, f64) {
    let t11523 = t5541 * t11522;
    let t11524 = t11523 * t8677;
    let t11526 = t5462 * t11522;
    let t11527 = t11526 * t8681;
    let t11529 = t3670 * t620;
    let t11531 = 0.15837668668010950386e-5_f64 * t11466 - 0.33765185592488808582e-6_f64 * t11469 - 0.33765185592488808582e-6_f64 * t11471 - 0.20011499994481700553e-7_f64 * t11475 - 0.20048078945540230096e-6_f64 * t11477 + 0.84540905957968605066e-6_f64 * t11481 + 0.11594181388521408695e-4_f64 * t11486 - 0.2318836277704281739e-4_f64 * t11490 + 0.2318836277704281739e-4_f64 * t11493 + 0.19323635647535681159e-6_f64 * t11497 - 0.343574241813184411e-6_f64 * t11501 + 0.11255061864162936194e-7_f64 * t11504 + 0.11255061864162936194e-6_f64 * t11506 - 0.15176747947735985782e-6_f64 * t11510 + 0.26984257851074582722e-6_f64 * t11515 + 0.24583187891642252608e-8_f64 * t11520 + 0.12650960286458333334e-5_f64 * t11524 + 0.12650960286458333334e-5_f64 * t11527 + 0.81088863580216065975e-3_f64 * t11529;
    (t11523, t11526, t11531)
}
