//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1133/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1133<F: Float>(t32948: F, t7427: F, t7573: F, t22333: F, t24344: F, t10889: F, t23176: F, t2017: F, t3488: F, t825: F, t22909: F, t25462: F, t787: F, t9824: F, t28231: F, t24885: F) -> (F, F, F, F, F, F, F, F) {
    let t32951 = 0.12423108009070322895e3 * t7427 * t7573 * t32948;
    let t32952 = t24344 * t22333;
    let t32953 = 0.29792074959875355558e-1 * t32952;
    let t32954 = t10889 * t23176;
    let t32955 = 0.59584149919750711116e-1 * t32954;
    let t32957 = t825 * t2017 * t3488;
    let t32958 = 0.59644551483876721719e0 * t32957;
    let t32959 = t10889 * t22909;
    let t32960 = 0.14896037479937677779e-1 * t32959;
    let t32962 = t787 * t25462 * t9824;
    let t32963 = 0.29792074959875355558e-1 * t32962;
    let t32968 = 0.31952438294933958064e0 * t28231;
    let t32969 = t787 * t24885;
    (t32951, t32953, t32955, t32958, t32960, t32963, t32968, t32969)
}
