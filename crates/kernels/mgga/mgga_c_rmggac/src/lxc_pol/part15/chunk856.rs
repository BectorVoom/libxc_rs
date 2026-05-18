//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 856/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk856<F: Float>(t8867: F, t8870: F, t8874: F, t8877: F, t8879: F, t8888: F, t9032: F, t9033: F, t9035: F, t9042: F, t9052: F, t9058: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t42293 = F::new(0.11974241701863808564e0) * t8867;
    let t42294 = F::new(2.0) * t8870;
    let t42296 = F::new(0.79828278012425390428e-1) * t8874;
    let t42297 = F::new(0.4726e1) * t8877;
    let t42298 = F::new(0.11974241701863808564e0) * t8879;
    let t42299 = F::new(2.0) * t8888;
    let t42300 = F::new(2.0) * t9032;
    let t42301 = F::new(0.11974241701863808564e0) * t9033;
    let t42306 = F::new(0.11974241701863808564e0) * t9035;
    let t42307 = F::new(0.85129199786595678796e-5) * t9042;
    let t42308 = F::new(0.85129199786595678796e-5) * t9052;
    let t42310 = F::new(0.11974241701863808564e0) * t9058;
    (t42293, t42294, t42296, t42297, t42298, t42299, t42300, t42301, t42306, t42307, t42308, t42310)
}
