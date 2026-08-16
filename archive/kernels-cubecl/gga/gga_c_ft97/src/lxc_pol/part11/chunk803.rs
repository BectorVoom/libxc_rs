//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 803/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk803<F: Float>(t2939: F, t684: F, t10864: F, t10835: F, t10838: F, t10839: F, t10841: F, t10843: F, t10847: F, t10852: F, t10855: F, t10859: F, t10862: F, t2265: F, t631: F) -> (F, F) {
    let t10865 = t684 * t2939;
    let t10866 = t10864 * t10865;
    let t10869 = -t10835 + t10838 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t10839 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t10841 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t10843 - t2265 * t10847 / F::cast_from(3.0_f64) - t631 * t10852 / F::cast_from(3.0_f64) + t2265 * t10855 / F::cast_from(6.0_f64) - t2265 * t10859 - t2265 * t10862 + F::cast_from(3.0_f64) * t2265 * t10866;
    (t10866, t10869)
}
