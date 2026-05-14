//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 588/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk588<F: Float>(t10024: F, t10867: F, t2714: F, t3040: F, t2718: F, t10850: F, t10853: F, t10855: F, t10859: F, t10862: F, t10864: F, t10866: F, t9812: F, t9815: F, t9822: F, t9826: F, t9832: F) -> (F,) {
    let t10868 = t10867 * t10024;
    let t10869 = 0.44688112439813033337e-1 * t10868;
    let t10871 = 0.35750489951850426669e0 * t2714 * t3040;
    let t10873 = 0.35750489951850426669e0 * t2718 * t3040;
    let t10874 = -t10850 + t10853 - t10855 - t10859 - t10862 + t10864 + t10866 - t10869 + t10871 + t10873 + t9812 + t9815 - t9822 + t9826 + t9832;
    (t10874,)
}
