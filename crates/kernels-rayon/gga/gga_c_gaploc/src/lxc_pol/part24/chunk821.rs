//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 821/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk821(t7402: f64, t7462: f64, t7523: f64, t7577: f64, t7629: f64, t7692: f64, t7750: f64, t7814: f64, t2590: f64, t747: f64, t1961: f64, t977: f64) -> (f64, f64, f64) {
    let t7817 = t7402 + t7462 + t7523 + t7577 + t7629 + t7692 + t7750 + t7814;
    let t7822 = t2590 * t747;
    let t7826 = t977 * t1961;
    (t7817, t7822, t7826)
}
