//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 899/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk899<F: Float>(t1317: F, t34483: F, t376: F, t25985: F, t32333: F, t22952: F, t22958: F, t136151: F, t136159: F, t34416: F, t3157: F, t7165: F, t1800: F, t28: F, t5665: F, t136226: F, t136229: F, t144892: F, t144895: F, t144899: F, t144904: F, t144908: F, t144912: F, t144917: F, t144919: F, t144923: F) -> (F, F, F, F, F, F, F) {
    let t144926 = t1317 * t376 * t34483;
    let t144928 = t32333 * t25985;
    let t144930 = t22952 * t22958 * t144928;
    let t144933 = t136159 * t136151 * t144928;
    let t144935 = t1317 * t376 * t34416;
    let t144938 = t7165 * t3157;
    let t144941 = t5665 * t28 * t1800 * t144938;
    let t144943 = -t144892 - 2.0 / 3.0 * t144895 - 6.0 * t144899 + 4.0 / 3.0 * t144904 - 6.0 * t144908 + 3.0 * t144912 + t144917 / 3.0 + 2.0 / 9.0 * t144919 - 4.0 / 3.0 * t144923 + t144926 / 6.0 - t136226 + t144930 / 6.0 - t144933 - t144935 / 3.0 + t136229 / 6.0 - t144941 / 2.0;
    (t144926, t144930, t144933, t144935, t144938, t144941, t144943)
}
