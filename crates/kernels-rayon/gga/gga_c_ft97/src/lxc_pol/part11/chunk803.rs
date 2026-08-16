//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 803/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk803(t2939: f64, t684: f64, t10864: f64, t10835: f64, t10838: f64, t10839: f64, t10841: f64, t10843: f64, t10847: f64, t10852: f64, t10855: f64, t10859: f64, t10862: f64, t2265: f64, t631: f64) -> (f64, f64) {
    let t10865 = t684 * t2939;
    let t10866 = t10864 * t10865;
    let t10869 = -t10835 + t10838 + 5.0_f64 / 3.0_f64 * t10839 + 4.0_f64 / 3.0_f64 * t10841 + 2.0_f64 / 3.0_f64 * t10843 - t2265 * t10847 / 3.0_f64 - t631 * t10852 / 3.0_f64 + t2265 * t10855 / 6.0_f64 - t2265 * t10859 - t2265 * t10862 + 3.0_f64 * t2265 * t10866;
    (t10866, t10869)
}
