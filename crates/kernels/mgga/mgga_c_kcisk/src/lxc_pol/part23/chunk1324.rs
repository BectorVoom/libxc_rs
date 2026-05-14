//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1324/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1324<F: Float>(t110859: F, t110861: F, t110863: F, t110865: F, t110868: F, t110870: F, t110873: F, t110876: F, t110879: F, t110881: F, t110883: F, t110885: F, t110887: F, t110890: F, t1010: F, t111490: F) -> (F,) {
    let t111505 = -3.0 / 8.0 * t110859 - 3.0 / 4.0 * t110861 - 9.0 / 4.0 * t110863 - 15.0 / 8.0 * t110865 - 3.0 / 4.0 * t110868 - t110870 / 32.0 + 15.0 / 4.0 * t110873 + 3.0 / 4.0 * t110876 + 3.0 / 4.0 * t110879 - 3.0 / 4.0 * t110881 - 3.0 * t110883 + 9.0 / 4.0 * t110885 + t110887 / 8.0 + 15.0 / 8.0 * t110890;
    let t111507 = t1010 * (t111490 + t111505);
    (t111507,)
}
