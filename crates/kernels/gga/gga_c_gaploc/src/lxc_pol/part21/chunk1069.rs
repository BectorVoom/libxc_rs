//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1069/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1069<F: Float>(t1897: F, t21455: F, t2580: F, t2958: F, t21460: F, t3487: F, t486: F, t7069: F, t21490: F, t10694: F, t29439: F, t24741: F, t5539: F, t9647: F, t10701: F, t25260: F, t2558: F) -> (F, F, F, F, F, F, F, F) {
    let t32139 = 0.30762104920568897134e-1 * t1897 * t2580 * t2958 * t21455;
    let t32143 = 0.15381052460284448567e-1 * t1897 * t2580 * t2958 * t21460;
    let t32145 = t3487 * t486 * t7069;
    let t32147 = 0.51270174867614828558e-2 * t21490 * t32145;
    let t32148 = t29439 * t10694;
    let t32149 = 0.1281754371690370714e-2 * t32148;
    let t32151 = t9647 * t5539 * t24741;
    let t32152 = 0.1281754371690370714e-2 * t32151;
    let t32153 = t29439 * t10701;
    let t32154 = 0.64087718584518535698e-3 * t32153;
    let t32158 = t9647 * t25260 * t2558;
    (t32139, t32143, t32145, t32147, t32149, t32152, t32154, t32158)
}
