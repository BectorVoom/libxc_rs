//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 432/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk432<F: Float>(t1691: F, t1821: F, t1819: F, t234: F, t704: F, t712: F, t740: F, t1719: F, t225: F, t739: F, t212: F, t716: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t1822 = t1821 * t1691;
    let t1823 = t1819 * t1822;
    let t1825 = F::cast_from(0.10254018858216406658e4_f64) * t234 * t1823;
    let t1826 = t704 * t712;
    let t1827 = t1826 * t740;
    let t1829 = F::cast_from(0.23392894490538584828e1_f64) * t234 * t1827;
    let t1830 = t225 * t1719;
    let t1831 = t739 * t1830;
    let t1833 = F::cast_from(0.11696447245269292414e1_f64) * t234 * t1831;
    let t1835 = F::new(1.0) / t716 / t212;
    (t1822, t1823, t1825, t1826, t1827, t1829, t1830, t1831, t1833, t1835)
}
