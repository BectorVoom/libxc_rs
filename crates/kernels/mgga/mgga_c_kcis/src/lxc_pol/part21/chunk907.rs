//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 907/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk907<F: Float>(t1035: F, t13786: F, t3061: F, t1045: F, t4547: F, t1680: F, t2980: F, t2938: F, t2939: F, t4722: F, t9660: F, t2988: F, t4718: F) -> (F, F, F, F, F, F) {
    let t13787 = t1035 * t13786;
    let t13790 = t3061 * t1035;
    let t13791 = t4547 * t1045;
    let t13796 = t1680 * t2980;
    let t13798 = F::new(2.0) * t2938 * t13796;
    let t13799 = t4722 * t2939;
    let t13801 = F::cast_from(0.96490945932906628932e2_f64) * t9660 * t13799;
    let t13802 = t4718 * t2988;
    (t13787, t13790, t13791, t13798, t13801, t13802)
}
