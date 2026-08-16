//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1338/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1338(t10455: f64, t4950: f64, t10140: f64, t1572: f64, t4673: f64, t10348: f64, t8155: f64, t31770: f64, t6824: f64, t20367: f64, t31775: f64, t10537: f64, t4379: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34900 = 0.95334639871601137784e0_f64 * t4950 * t10455;
    let t34903 = 0.95334639871601137784e0_f64 * t1572 * t4673 * t10140;
    let t34905 = 0.14300195980740170668e1_f64 * t8155 * t10348;
    let t34910 = 0.95334639871601137784e0_f64 * t6824 * t31770;
    let t34912 = 0.47667319935800568892e0_f64 * t20367 * t31775;
    let t34913 = t4379 * t10537;
    (t34900, t34903, t34905, t34910, t34912, t34913)
}
