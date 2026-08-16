//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1161/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1161(t2761: f64, t6295: f64, t6525: f64, t10163: f64, t29874: f64, t2321: f64, t26673: f64, t9074: f64, t26629: f64, t10141: f64, t6313: f64, t10145: f64, t6305: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31487 = t6525 * t2761 * t6295;
    let t31488 = 0.11856252764865062333e-2_f64 * t31487;
    let t31489 = t29874 * t10163;
    let t31490 = 0.23712505529730124666e-2_f64 * t31489;
    let t31492 = t9074 * t26673 * t2321;
    let t31493 = 0.11856252764865062333e-2_f64 * t31492;
    let t31495 = t9074 * t26629 * t2321;
    let t31496 = 0.23712505529730124666e-2_f64 * t31495;
    let t31498 = 0.15176003539027279786e0_f64 * t6313 * t10141;
    let t31522 = 0.1138200265427045984e0_f64 * t6305 * t10145;
    (t31488, t31490, t31493, t31496, t31498, t31522)
}
