//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1089/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1089<F: Float>(t19947: F, t28024: F, t13106: F, t1813: F, t28050: F, t28059: F, t20178: F, t7748: F, t19950: F, t26896: F, t20159: F, t283: F, t6681: F, t7755: F, t2825: F, t6728: F) -> (F, F, F, F, F, F, F, F) {
    let t99904 = t28024 * t19947;
    let t99906 = t13106 * t1813;
    let t99908 = t28059 * t28050;
    let t99910 = t7748 * t20178;
    let t99912 = t26896 * t19950;
    let t99914 = t26896 * t20159;
    let t99916 = t6681 * t283;
    let t99917 = t99916 * t7755;
    let t99919 = t2825 * t6728;
    (t99904, t99906, t99908, t99910, t99912, t99914, t99917, t99919)
}
