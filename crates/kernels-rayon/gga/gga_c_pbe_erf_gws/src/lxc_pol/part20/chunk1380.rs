//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1380/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1380(t2409: f64, t36089: f64, t3959: f64, t14001: f64, t15331: f64, t1178: f64, t12169: f64, t371: f64, t3983: f64, t13953: f64, t15345: f64, t35654: f64) -> (f64, f64, f64, f64, f64) {
    let t57694 = t3959 * t2409 * t36089;
    let t57696 = t14001 * t15331;
    let t57700 = t3983 * t371 * t1178 * t12169;
    let t57702 = t13953 * t15345;
    let t57705 = t3959 * t2409 * t35654;
    (t57694, t57696, t57700, t57702, t57705)
}
