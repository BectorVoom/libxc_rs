//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 915/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk915<F: Float>(t23925: F, t558: F, t28: F, t89: F, t376: F, t5921: F, t2075: F, t5778: F, t23669: F, t23674: F, t23887: F, t23890: F, t23895: F, t23899: F, t23903: F, t23907: F, t23912: F, t23914: F, t23918: F, t23920: F, t23924: F) -> (F, F, F, F, F, F) {
    let t23926 = t23925 * t558;
    let t23927 = t28 * t23926;
    let t23928 = t89 * t23927;
    let t23930 = t376 * t5921;
    let t23931 = t89 * t23930;
    let t23933 = t5778 * t2075;
    let t23934 = t28 * t23933;
    let t23935 = t89 * t23934;
    let t23937 = -2.0 / 3.0 * t23669 - t23674 / 6.0 - t23887 / 2.0 + t23890 / 3.0 - 2.0 / 3.0 * t23895 - t23899 + 2.0 / 3.0 * t23903 + t23907 / 3.0 + 2.0 / 9.0 * t23912 - 2.0 / 9.0 * t23914 - t23918 + 2.0 / 3.0 * t23920 - t23924 + 4.0 * t23928 - 4.0 / 3.0 * t23931 + 2.0 * t23935;
    (t23926, t23928, t23931, t23933, t23935, t23937)
}
