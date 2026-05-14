//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 898/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk898<F: Float>(t19895: F, t19897: F, t19899: F, t19902: F, t19906: F, t19909: F, t19912: F, t19914: F, t19916: F, t19918: F, t19920: F, t19922: F, t19924: F, t19927: F, t19929: F, t19932: F, t19935: F, t19938: F, t19941: F) -> (F,) {
    let t20769 = -0.44965277777777777777e-2 * t19895 - 0.14388888888888888889e0 * t19897 - 0.1875e0 * t19899 - 0.4046875e-1 * t19902 - 0.101171875e-1 * t19906 - 0.44965277777777777777e-2 * t19909 + 0.23981481481481481481e-1 * t19912 + 0.101171875e-1 * t19914 + 0.20234375e-1 * t19916 + 0.125e0 * t19918 - 0.125e0 * t19920 - 0.20234375e-1 * t19922 - 0.9375e-1 * t19924 - 0.125e0 * t19927 - 0.10791666666666666667e0 * t19929 - 0.5625e0 * t19932 + 0.20833333333333333333e-1 * t19935 + 0.27777777777777777777e-1 * t19938 - 0.26979166666666666667e-1 * t19941;
    (t20769,)
}
