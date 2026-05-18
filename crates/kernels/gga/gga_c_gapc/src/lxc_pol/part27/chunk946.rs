//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 946/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk946<F: Float>(t11522: F, t5541: F, t8677: F, t5462: F, t8681: F, t3670: F, t620: F, t11466: F, t11469: F, t11471: F, t11475: F, t11477: F, t11481: F, t11486: F, t11490: F, t11493: F, t11497: F, t11501: F, t11504: F, t11506: F, t11510: F, t11515: F, t11520: F) -> (F, F, F) {
    let t11523 = t5541 * t11522;
    let t11524 = t11523 * t8677;
    let t11526 = t5462 * t11522;
    let t11527 = t11526 * t8681;
    let t11529 = t3670 * t620;
    let t11531 = F::new(0.15837668668010950386e-5) * t11466 - F::new(0.33765185592488808582e-6) * t11469 - F::new(0.33765185592488808582e-6) * t11471 - F::new(0.20011499994481700553e-7) * t11475 - F::new(0.20048078945540230096e-6) * t11477 + F::new(0.84540905957968605066e-6) * t11481 + F::new(0.11594181388521408695e-4) * t11486 - F::new(0.2318836277704281739e-4) * t11490 + F::new(0.2318836277704281739e-4) * t11493 + F::new(0.19323635647535681159e-6) * t11497 - F::new(0.343574241813184411e-6) * t11501 + F::new(0.11255061864162936194e-7) * t11504 + F::new(0.11255061864162936194e-6) * t11506 - F::new(0.15176747947735985782e-6) * t11510 + F::new(0.26984257851074582722e-6) * t11515 + F::new(0.24583187891642252608e-8) * t11520 + F::new(0.12650960286458333334e-5) * t11524 + F::new(0.12650960286458333334e-5) * t11527 + F::new(0.81088863580216065975e-3) * t11529;
    (t11523, t11526, t11531)
}
