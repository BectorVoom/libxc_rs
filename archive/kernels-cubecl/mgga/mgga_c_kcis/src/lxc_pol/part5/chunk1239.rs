//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1239/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1239<F: Float>(t19895: F, t19897: F, t19899: F, t19902: F, t19906: F, t19909: F, t19912: F, t19914: F, t19916: F, t19918: F, t19920: F, t19922: F, t19924: F, t19927: F, t19929: F, t19932: F, t19935: F, t19938: F, t19941: F) -> F {
    let t20769 = -F::cast_from(0.44965277777777777777e-2_f64) * t19895 - F::cast_from(0.14388888888888888889e0_f64) * t19897 - F::cast_from(0.1875e0_f64) * t19899 - F::cast_from(0.4046875e-1_f64) * t19902 - F::cast_from(0.101171875e-1_f64) * t19906 - F::cast_from(0.44965277777777777777e-2_f64) * t19909 + F::cast_from(0.23981481481481481481e-1_f64) * t19912 + F::cast_from(0.101171875e-1_f64) * t19914 + F::cast_from(0.20234375e-1_f64) * t19916 + F::cast_from(0.125e0_f64) * t19918 - F::cast_from(0.125e0_f64) * t19920 - F::cast_from(0.20234375e-1_f64) * t19922 - F::cast_from(0.9375e-1_f64) * t19924 - F::cast_from(0.125e0_f64) * t19927 - F::cast_from(0.10791666666666666667e0_f64) * t19929 - F::cast_from(0.5625e0_f64) * t19932 + F::cast_from(0.20833333333333333333e-1_f64) * t19935 + F::cast_from(0.27777777777777777777e-1_f64) * t19938 - F::cast_from(0.26979166666666666667e-1_f64) * t19941;
    t20769
}
