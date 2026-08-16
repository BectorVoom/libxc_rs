//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1023/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1023<F: Float>(t1871: F, t25893: F, t25894: F, t32094: F, t22952: F, t25888: F, t8411: F, t144829: F, t144832: F, t144836: F, t144840: F, t144844: F, t144848: F, t144851: F, t144855: F, t144859: F, t144863: F, t144866: F, t144870: F, t144874: F, t144878: F) -> (F, F, F) {
    let t144882 = t25893 * t1871 * t32094 * t25894;
    let t144886 = t22952 * t8411 * t32094 * t25888;
    let t144888 = -t144829 / F::cast_from(12.0_f64) - t144832 - F::cast_from(20.0_f64) * t144836 + F::cast_from(8.0_f64) * t144840 - t144844 / F::cast_from(12.0_f64) + t144848 - t144851 / F::cast_from(3.0_f64) - t144855 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t144859 - t144863 + F::cast_from(12.0_f64) * t144866 - F::cast_from(6.0_f64) * t144870 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t144874 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t144878 + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t144882 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t144886;
    (t144882, t144886, t144888)
}
