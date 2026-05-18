//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 878/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk878<F: Float>(t8878: F, t8881: F, t8886: F, t8889: F, t8891: F, t8898: F, t8901: F, t8904: F, t8908: F, t8913: F, t8917: F, t8919: F, t8922: F) -> F {
    let t10633 = -F::new(0.16882592796244404291e-6) * t8878 - F::new(0.20011499994481700554e-7) * t8881 - F::new(0.98481791311425691698e-7) * t8886 + F::new(0.24761136101158459626e-5) * t8889 - F::new(0.24326659074064819793e-2) * t8891 - F::new(0.75091666377929252765e-6) * t8898 + F::new(0.4637672555408563478e-4) * t8901 + F::new(0.4048307291666666667e-4) * t8904 - F::new(0.17989505234049721814e-7) * t8908 + F::new(0.23989005229605304038e-7) * t8913 - F::new(0.9004049491330348955e-7) * t8917 - F::new(0.88164651269276333518e-6) * t8919 + F::new(0.51491428373437201896e-5) * t8922;
    t10633
}
