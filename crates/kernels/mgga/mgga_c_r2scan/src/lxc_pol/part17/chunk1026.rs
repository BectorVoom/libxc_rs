//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1026/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1026<F: Float>(t12894: F, t333: F, t335: F, t337: F, t339: F, t341: F, t1020: F, t1135: F, t1137: F, t12890: F, t12892: F, t2956: F, t343: F, t3765: F) -> (F, F, F, F, F, F) {
    let t12895 = t333 * t12894;
    let t12897 = t335 * t12894;
    let t12899 = t337 * t12894;
    let t12901 = t339 * t12894;
    let t12903 = t341 * t12894;
    let t12908 = F::new(0.1550653405116e2) * t1135 * t2956 - F::new(0.4355305902528e1) * t3765 * t1020 - F::new(0.2177652951264e1) * t1137 * t2956 - F::new(0.8704e0) * t12890 - F::new(0.17408e1) * t12892 - F::new(0.8704e0) * t12895 - F::new(0.4607056813647e1) * t12897 + F::new(0.122462410087e2) * t12899 - F::new(0.957855118103e1) * t12901 + F::new(0.3101306810232e1) * t12903 - F::new(0.362942158544e0) * t343 * t12894 - F::new(0.64e0) * t12894;
    (t12895, t12897, t12899, t12901, t12903, t12908)
}
