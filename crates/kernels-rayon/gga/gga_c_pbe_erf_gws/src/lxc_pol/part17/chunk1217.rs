//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1217/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1217(t12276: f64, t50832: f64, t1167: f64, t2182: f64, t13751: f64, t13756: f64, t14149: f64, t14153: f64, t14383: f64, t14821: f64, t2423: f64, t2494: f64, t3946: f64, t4062: f64, t4063: f64, t50833: f64, t50835: f64, t52052: f64, t52763: f64, t52767: f64, t52774: f64, t52775: f64, t52782: f64) -> f64 {
    let t52789 = 6.0_f64 * t50832 * t12276;
    let t52791 = t1167 * t2182;
    let t52798 = -t1167 * t4062 * t52052 + 6.0_f64 * t13751 * t2494 * t3946 - 6.0_f64 * t13756 * t4063 * t52791 - 6.0_f64 * t14149 * t14383 * t3946 + 2.0_f64 * t14153 * t4062 * t52763 - t14821 * t2423 * t4062 - 6.0_f64 * t3946 * t4063 * t52767 - 6.0_f64 * t3946 * t4063 * t52782 + 12.0_f64 * t52774 * t52775 - 6.0_f64 * t50833 + 6.0_f64 * t50835 - t52789;
    t52798
}
