//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1152/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1152<F: Float>(t1020: F, t1081: F, t1085: F, t1087: F, t1089: F, t11930: F, t12659: F, t2410: F, t2956: F, t2958: F, t333: F, t335: F, t337: F, t3386: F, t339: F, t341: F, t343: F, t3648: F, t42616: F, t839: F, t9707: F, t9709: F, t9715: F) -> F {
    let t42742 = F::new(0.18607840861392e3) * t1085 * t9715 + F::new(0.12405227240928e3) * t1087 * t9709 - F::new(0.4355305902528e2) * t1087 * t9715 - F::new(0.2177652951264e2) * t1089 * t9709 + F::new(0.122462410087e2) * t337 * t42616 - F::new(0.957855118103e1) * t339 * t42616 + F::new(0.3101306810232e1) * t341 * t42616 - F::new(0.362942158544e0) * t343 * t42616 - F::new(0.8704e0) * t839 * t12659 - F::new(0.8704e0) * t333 * t42616 - F::new(0.4607056813647e1) * t335 * t42616 - F::new(0.9214113627294e1) * t2958 * t3386 - F::new(0.8704e0) * t9707 * t1081 - F::new(0.8704e0) * t2956 * t3386 - F::new(0.17408e1) * t2410 * t3648 - F::new(0.17408e1) * t1020 * t11930;
    t42742
}
