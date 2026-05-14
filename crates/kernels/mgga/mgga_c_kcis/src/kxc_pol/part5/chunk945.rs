//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 945/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk945<F: Float>(t15573: F, t5331: F, t1251: F, t13391: F, t13408: F, t14078: F, t14081: F, t14085: F, t14104: F, t14567: F, t14576: F, t14607: F, t1864: F, t3668: F, t12274: F, t2003: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t15574 = t15573 * t5331;
    let t15576 = t1251 * t15574 / 144.0;
    let t15602 = 0.15476481481481481481e-2 * t13391;
    let t15607 = 0.15476481481481481481e-2 * t13408;
    let t15632 = 0.23214722222222222222e-2 * t14078;
    let t15638 = 0.30952962962962962962e-2 * t14081;
    let t15639 = 0.15476481481481481481e-2 * t14085;
    let t15648 = 0.15476481481481481481e-2 * t14104;
    let t15659 = 0.23214722222222222222e-2 * t14567;
    let t15662 = 0.15476481481481481481e-2 * t14576;
    let t15671 = 0.15476481481481481481e-2 * t14607;
    let t15692 = t1864 * t3668;
    let t15800 = t12274 * t2003;
    (t15576, t15602, t15607, t15632, t15638, t15639, t15648, t15659, t15662, t15671, t15692, t15800)
}
