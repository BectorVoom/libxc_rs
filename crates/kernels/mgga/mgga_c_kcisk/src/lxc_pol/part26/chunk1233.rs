//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1233/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1233<F: Float>(t110859: F, t110861: F, t110863: F, t110865: F, t110868: F, t110870: F, t110873: F, t110876: F, t110879: F, t110881: F, t110883: F, t110885: F, t110887: F, t110890: F, t1139: F, t32678: F) -> (F, F) {
    let t110892 = -0.5625e0 * t110859 - 0.19425e1 * t110861 - 0.3375e1 * t110863 - 0.485625e1 * t110865 - 0.1125e1 * t110868 - 0.809375e-1 * t110870 + 0.97125e1 * t110873 + 0.19425e1 * t110876 + 0.1125e1 * t110879 - 0.1125e1 * t110881 - 0.45e1 * t110883 + 0.3375e1 * t110885 + 0.1875e0 * t110887 + 0.485625e1 * t110890;
    let t110898 = t32678 * t1139;
    (t110892, t110898)
}
