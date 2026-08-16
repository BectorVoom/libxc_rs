//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 927/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk927<F: Float>(t8878: F, t8881: F, t8886: F, t8889: F, t8891: F, t8898: F, t8901: F, t8904: F, t8908: F, t8913: F, t8917: F, t8919: F, t8922: F) -> F {
    let t10633 = -F::cast_from(0.16882592796244404291e-6_f64) * t8878 - F::cast_from(0.20011499994481700554e-7_f64) * t8881 - F::cast_from(0.98481791311425691698e-7_f64) * t8886 + F::cast_from(0.24761136101158459626e-5_f64) * t8889 - F::cast_from(0.24326659074064819793e-2_f64) * t8891 - F::cast_from(0.75091666377929252765e-6_f64) * t8898 + F::cast_from(0.4637672555408563478e-4_f64) * t8901 + F::cast_from(0.4048307291666666667e-4_f64) * t8904 - F::cast_from(0.17989505234049721814e-7_f64) * t8908 + F::cast_from(0.23989005229605304038e-7_f64) * t8913 - F::cast_from(0.9004049491330348955e-7_f64) * t8917 - F::cast_from(0.88164651269276333518e-6_f64) * t8919 + F::cast_from(0.51491428373437201896e-5_f64) * t8922;
    t10633
}
