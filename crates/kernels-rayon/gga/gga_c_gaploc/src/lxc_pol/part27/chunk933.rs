//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 933/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk933(t3347: f64, t6313: f64, t3344: f64, t484: f64, t874: f64, t986: f64) -> (f64, f64, f64) {
    let t10238 = 0.1138200265427045984e0_f64 * t6313 * t3347;
    let t10239 = t484 * t3344;
    let t10240 = 0.15808337019820083111e-2_f64 * t10239;
    let t10241 = t874 * t986;
    (t10238, t10240, t10241)
}
