//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1282/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1282<F: Float>(t2099: F, t2945: F, t9590: F, t17869: F, t2003: F, t2104: F, t21417: F, t21485: F, t21494: F, t21496: F, t21499: F, t21527: F, t21538: F, t21540: F, t2899: F, t2916: F, t2922: F, t300: F, t5984: F, t655: F, t758: F, t9161: F, t9259: F, t9263: F, t9292: F, t9572: F) -> (F,) {
    let t25275 = t2945 * t2099 * t9590;
    let t25285 = 0.19055119163586549765e-3 * t17869 + 0.6097638132347695925e-2 * t21485 + 0.30488190661738479624e-2 * t21494 + 0.60976381323476959249e-2 * t21496 - 0.3811023832717309953e-3 * t21499 + 0.91464571985215438874e-2 * t5984 * t9572 + 0.51448821741683684366e-2 * t2104 * t300 * t2003 * t2916 * t9259 - 0.34299214494455789578e-2 * t2899 * t21417 * t9263 + 0.17149607247227894789e-2 * t2922 * t21417 * t9292 + 0.17149607247227894789e-2 * t25275 + 0.25724410870841842184e-2 * t2945 * t758 * t2003 * t9161 * t655 - t21527 / 144.0 - 5.0 / 648.0 * t21538 - 11.0 / 162.0 * t21540;
    (t25285,)
}
