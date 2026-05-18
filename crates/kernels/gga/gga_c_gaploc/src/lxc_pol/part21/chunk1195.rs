//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1195/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1195<F: Float>(t24722: F, t2508: F, t2541: F, t1897: F, t2580: F, t7068: F, t8469: F, t21455: F, t2958: F, t21460: F, t3487: F, t486: F, t7069: F) -> (F, F, F, F, F) {
    let t32131 = F::new(0.53833683610995569986e-1) * t2508 * t2541 * t24722;
    let t32135 = F::new(0.30762104920568897134e-1) * t1897 * t2580 * t8469 * t7068;
    let t32139 = F::new(0.30762104920568897134e-1) * t1897 * t2580 * t2958 * t21455;
    let t32143 = F::new(0.15381052460284448567e-1) * t1897 * t2580 * t2958 * t21460;
    let t32145 = t3487 * t486 * t7069;
    (t32131, t32135, t32139, t32143, t32145)
}
