//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1190/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1190<F: Float>(t1691: F, t1719: F, t1917: F, t1923: F, t1945: F, t1966: F, t1990: F, t2008: F, t2030: F, t206: F, t208: F, t21420: F, t21829: F, t21832: F, t21837: F, t21843: F, t21846: F, t21858: F, t224: F, t5269: F, t5435: F, t5439: F, t5524: F, t5537: F, t5748: F, t625: F, t668: F, t674: F, t687: F, t712: F, t718: F) -> (F,) {
    let t21868 = -0.14897091720750739287e6 * t5748 * t206 * t2008 * t1966 * t1923 + t21829 + t21832 - 0.12467418556090653906e4 * t1945 * t712 * t5439 + t21837 + 18.0 * t687 * t208 * t21420 + t21843 + t21846 + 0.54649562515291533626e6 * t5537 * t224 * t5269 * t1691 * t1719 + 0.42107210082969452692e2 * t718 * t712 * t5435 - t21858 + 0.43374325201206959368e-1 * t625 * t1990 * t1917 - 0.68493333333333333332e-1 * t625 * t668 * t5524 - 12.0 * t674 * t2030 * t1966;
    (t21868,)
}
