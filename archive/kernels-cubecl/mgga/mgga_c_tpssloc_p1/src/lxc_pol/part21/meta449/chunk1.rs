//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2000/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2000<F: Float>(t11692: F, t1174: F, t11834: F, t15686: F, t15691: F, t15699: F, t15704: F, t15710: F, t15714: F, t15717: F, t15719: F, t15722: F, t15723: F, t3552: F, t3557: F, t3562: F, t3577: F, t488: F, t4889: F) -> F {
    let t15726 = t1174 * t15686 / F::cast_from(36.0_f64) - t15691 + t4889 * t3552 / F::cast_from(108.0_f64) + t4889 * t3557 / F::cast_from(54.0_f64) - t4889 * t3562 / F::cast_from(81.0_f64) + t15699 + t11692 * t15704 / F::cast_from(2304.0_f64) - t3577 * t15710 / F::cast_from(1152.0_f64) + t11834 + F::cast_from(5.0_f64) / F::cast_from(13824.0_f64) * t3577 * t15714 + t15717 / F::cast_from(2592.0_f64) - t15719 / F::cast_from(13824.0_f64) - t15722 - t15723 * t488 / F::cast_from(576.0_f64);
    t15726
}
