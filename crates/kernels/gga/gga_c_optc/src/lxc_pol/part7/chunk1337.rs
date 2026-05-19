//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1337/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1337<F: Float>(t2972: F, t393: F, t2975: F, t2916: F, t3029: F, t1067: F, t1074: F, t1075: F, t1076: F, t26237: F, t26240: F, t26242: F, t26245: F, t26251: F, t26455: F, t26578: F, t26588: F, t26593: F, t26611: F, t26626: F, t26642: F, t26657: F, t2917: F, t2930: F, t2935: F, t2968: F, t2974: F, t2976: F, t2977: F, t3036: F, t8765: F, t8781: F, t8786: F, t8791: F, t8799: F, t8842: F, t8843: F, t8850: F, t8851: F) -> F {
    let t26663 = t2972 * t2972;
    let t26665 = t393 / t26663;
    let t26666 = t2975 * t2975;
    let t26667 = F::new(1.0) / t26666;
    let t26671 = t3029 * t2916;
    let t26686 = -F::cast_from(0.62336721237753107879e3_f64) * t8765 * t8791 * t2917 - t26237 - t26240 - t26242 - t26245 + t26251 - t26455 + F::new(4.0) * t2930 * t8843 + F::cast_from(0.82765347514623860983e4_f64) * t26588 * t8851 - F::cast_from(0.24829604254387158296e5_f64) * t26593 * t26578 * t8850 + F::new(1.0) * t1067 * (t26611 + t26626 + t26642 + t26657) * t1075 + F::cast_from(0.19965908856856833625e6_f64) * t26665 * t26578 * t26667 - F::cast_from(0.70178680769462448852e1_f64) * t26671 * t3036 - F::new(8.0) * t2935 * t1076 * t8842 - F::cast_from(0.11579285944033451271e4_f64) * t8786 * t2977 * t2968 + F::cast_from(0.38597619813444837568e3_f64) * t8781 * t8799 + F::cast_from(0.1286587327114827919e3_f64) * t2974 * t8842 * t2976 * t1074;
    t26686
}
