//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 798/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk798<F: Float>(t9513: F, t9516: F, t9518: F, t9521: F, t9523: F, t9526: F, t9530: F, t9533: F, t9536: F, t9539: F, t9541: F, t9544: F, t9546: F, t9556: F, t9558: F, t9561: F, t9565: F, t9568: F, t9570: F, t9572: F, t9579: F, t9581: F, t9584: F, t9587: F, t9589: F, t9592: F) -> (F, F) {
    let t10856 = 0.12974218172834570556e-1 * t9513 + 0.27801896084645508334e-2 * t9516 + 0.55603792169291016668e-2 * t9518 - 0.14492726735651760868e-5 * t9521 - 0.10136107947527008247e-3 * t9523 - 0.10136107947527008247e-3 * t9526 + 0.30361328125000000002e-3 * t9530 - 0.10120442708333333334e-3 * t9533 + 0.6746961805555555556e-5 * t9536 + 0.28985453471303521736e-5 * t9539 + 0.2471588561924985691e-3 * t9541 + 0.2471588561924985691e-3 * t9544 - 0.6746961805555555556e-5 * t9546;
    let t10872 = -0.98393192997685185193e-6 * t9556 - 0.33816362383187442026e-4 * t9558 + 0.14492726735651760868e-5 * t9561 + 0.16882049790461501058e-6 * t9565 + 0.33764099580923002116e-6 * t9568 - 0.61320337121513228211e-3 * t9570 + 0.9275345110817126956e-4 * t9572 - 0.16882049790461501058e-6 * t9579 - 0.19678638599537037038e-4 * t9581 + 0.76020809606452561851e-3 * t9584 + 0.28985453471303521736e-5 * t9587 + 0.67632724766374884052e-4 * t9589 - 0.28985453471303521736e-5 * t9592;
    (t10856, t10872)
}
