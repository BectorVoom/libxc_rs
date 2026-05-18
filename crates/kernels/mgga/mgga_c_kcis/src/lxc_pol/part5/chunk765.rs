//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 765/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk765<F: Float>(t5741: F, t5894: F, t589: F, t1505: F, t2016: F, t1555: F, t2069: F, t4184: F, t4189: F, t4291: F, t576: F, t251: F, t4301: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5895 = t5741 + t5894;
    let t5896 = t5895 * t589;
    let t5897 = t2016 * t1505;
    let t5898 = t5897 * t1555;
    let t5899 = t4184 * t2069;
    let t5900 = t2069 * t1555;
    let t5902 = F::new(2.0) * t4189 * t5900;
    let t5903 = t576 * t4291;
    let t5904 = t251 * t4301;
    (t5895, t5896, t5897, t5898, t5899, t5900, t5902, t5903, t5904)
}
