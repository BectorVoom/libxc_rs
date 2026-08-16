//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta308 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1352;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1353;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta308(t3069: f64, t3180: f64, t3036: f64, t67: f64, t3067: f64, t3186: f64, t3062: f64, t820: f64, t3200: f64, t3051: f64, t3072: f64, t3070: f64, t1005: f64, t3082: f64, t1036: f64, t3094: f64, t3089: f64, t248: f64, t2780: f64, t1041: f64, t121: f64, t3061: f64, t2771: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10390, t10401, t10403, t10408, t10413, t10422, t10424) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1352(t3069, t3180, t3036, t67, t3067, t3186, t3062, t820, t3200, t3051, t3072, t3070);
        let (t10436, t10441, t10449, t10455, t10459) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1353(t1005, t3082, t1036, t3094, t3089, t248, t2780, t3051, t1041, t121, t3061, t2771);
    (t10390, t10401, t10403, t10408, t10413, t10422, t10424, t10436, t10441, t10449, t10455, t10459)
}
