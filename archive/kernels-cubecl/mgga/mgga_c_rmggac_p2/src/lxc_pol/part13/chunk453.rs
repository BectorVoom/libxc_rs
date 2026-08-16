//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 453/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk453<F: Float>(t1570: F, t1574: F, t309: F, t3901: F, t4858: F, t4861: F, t4862: F, t4865: F, t4868: F, t4871: F, t4879: F, t4882: F, t4883: F, t4886: F, t4889: F, t4892: F, t538: F, t544: F, t804: F, t822: F, t826: F, t87: F, t98: F) -> F {
    let t4895 = F::cast_from(400.0_f64) / F::cast_from(27.0_f64) * t804 * t538 - F::cast_from(200.0_f64) / F::cast_from(27.0_f64) * t309 * t1570 - F::cast_from(100.0_f64) / F::cast_from(9.0_f64) * t309 * t1574 - F::cast_from(20.0_f64) / F::cast_from(27.0_f64) * t87 * t4858 + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t4861 * t4862 + F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t87 * t4865 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t87 * t4868 - F::cast_from(10.0_f64) * t87 * t4871 - F::cast_from(100.0_f64) / F::cast_from(27.0_f64) * t544 * t822 - F::cast_from(50.0_f64) / F::cast_from(9.0_f64) * t544 * t826 - F::cast_from(20.0_f64) / F::cast_from(27.0_f64) * t98 * t4879 - F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t4882 * t4883 + F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t98 * t4886 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t98 * t4889 + F::cast_from(10.0_f64) * t98 * t4892 + t3901;
    t4895
}
