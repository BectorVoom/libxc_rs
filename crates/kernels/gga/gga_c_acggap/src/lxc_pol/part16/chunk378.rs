//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 378/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk378<F: Float>(t1382: F, t1384: F, t1386: F, t1391: F, t1358: F, t1710: F, t1712: F, t684: F, t693: F, t805: F, t905: F, t659: F, t708: F, t711: F, t714: F, t717: F, t753: F, t757: F, t764: F, t774: F, t782: F, t809: F, t914: F) -> (F, F) {
    let t1820 = F::new(0.11696447245269292414e1) * t1382;
    let t1821 = F::new(8.0) * t1384;
    let t1822 = F::new(8.0) * t1386;
    let t1823 = F::new(2.0) * t1391;
    let t1824 = F::new(0.36622894612013090108e-3) * t1358;
    let t1825 = t1712 + t1710 - t1820 - t1821 - t1822 + t1823 - t1824 - t684 - t693 + t805 - t905;
    let t1826 = -t708 - t764 + t711 + t714 + t717 - t753 + t774 + t782 + t659 + t809 + t914 - t757;
    (t1825, t1826)
}
