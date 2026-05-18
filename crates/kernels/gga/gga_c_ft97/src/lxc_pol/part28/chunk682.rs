//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 682/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk682<F: Float>(t167: F, t2185: F, t26909: F, t1882: F, t6710: F, t6627: F, t8392: F, t3052: F, t5942: F, t2210: F, t11593: F, t1901: F, t26868: F, t26872: F, t26876: F, t26880: F, t26885: F, t26890: F, t26894: F, t26899: F, t26902: F, t26906: F, t446: F) -> (F, F) {
    let t26911 = t2185 * t167 * t26909;
    let t26914 = t1882 * t6710;
    let t26916 = t8392 * t6627;
    let t26918 = t5942 * t3052;
    let t26919 = t2210 * t26918;
    let t26922 = t1901 * t26868 / F::new(9.0) - t446 * t26872 / F::new(9.0) + t446 * t26876 / F::new(3.0) + t446 * t26880 / F::new(3.0) + t446 * t26885 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t446 * t26890 + F::new(2.0) / F::new(3.0) * t446 * t26894 + t446 * t26899 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t446 * t26902 + F::new(2.0) / F::new(3.0) * t446 * t26906 + F::new(2.0) / F::new(3.0) * t446 * t26911 - F::new(2.0) / F::new(9.0) * t26914 - t26916 / F::new(27.0) + F::new(2.0) / F::new(9.0) * t11593 * t26919;
    (t26918, t26922)
}
