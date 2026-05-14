//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 831/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk831<F: Float>(t8878: F, t8881: F, t8886: F, t8889: F, t8891: F, t8898: F, t8901: F, t8904: F, t8908: F, t8913: F, t8917: F, t8919: F, t8922: F, t8927: F, t8930: F, t8933: F, t8935: F, t8938: F, t8941: F, t8943: F, t8945: F, t8952: F, t8961: F, t8963: F, t8966: F, t8969: F) -> (F, F) {
    let t10633 = -0.16882592796244404291e-6 * t8878 - 0.20011499994481700554e-7 * t8881 - 0.98481791311425691698e-7 * t8886 + 0.24761136101158459626e-5 * t8889 - 0.24326659074064819793e-2 * t8891 - 0.75091666377929252765e-6 * t8898 + 0.4637672555408563478e-4 * t8901 + 0.4048307291666666667e-4 * t8904 - 0.17989505234049721814e-7 * t8908 + 0.23989005229605304038e-7 * t8913 - 0.9004049491330348955e-7 * t8917 - 0.88164651269276333518e-6 * t8919 + 0.51491428373437201896e-5 * t8922;
    let t10648 = -0.38647271295071362317e-7 * t8927 + 0.14492726735651760868e-5 * t8930 + 0.2471588561924985691e-3 * t8933 + 0.74218967013888888897e-4 * t8935 + 0.13900948042322754167e-3 * t8938 - 0.74147656857749570729e-3 * t8941 + 0.16682738775705804733e-3 * t8943 - 0.1349435763888888889e-4 * t8945 - 0.19679271556712962965e-5 * t8952 + 0.86096813060619212971e-6 * t8961 + 0.2471588561924985691e-3 * t8963 + 0.28985453471303521736e-5 * t8966 + 0.4048307291666666667e-4 * t8969;
    (t10633, t10648)
}
