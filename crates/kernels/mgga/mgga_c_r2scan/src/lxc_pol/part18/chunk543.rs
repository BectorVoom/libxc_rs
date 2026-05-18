//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 543/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk543<F: Float>(t2956: F, t333: F, t2958: F, t335: F, t337: F, t339: F, t341: F, t343: F, t1035: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t2960 = t333 * t2956;
    let t2962 = t333 * t2958;
    let t2964 = t335 * t2956;
    let t2966 = t335 * t2958;
    let t2968 = t337 * t2956;
    let t2970 = t337 * t2958;
    let t2972 = t339 * t2956;
    let t2974 = t339 * t2958;
    let t2976 = t341 * t2956;
    let t2982 = -F::new(0.64e0) * t2956 - F::new(0.8704e0) * t2958 - F::new(0.8704e0) * t2960 - F::new(0.9214113627294e1) * t2962 - F::new(0.4607056813647e1) * t2964 + F::new(0.367387230261e2) * t2966 + F::new(0.122462410087e2) * t2968 - F::new(0.3831420472412e2) * t2970 - F::new(0.957855118103e1) * t2972 + F::new(0.1550653405116e2) * t2974 + F::new(0.3101306810232e1) * t2976 - F::new(0.2177652951264e1) * t341 * t2958 - F::new(0.362942158544e0) * t343 * t2956;
    let t2983 = t1035 * t1035;
    (t2960, t2962, t2964, t2966, t2968, t2970, t2972, t2974, t2976, t2982, t2983)
}
